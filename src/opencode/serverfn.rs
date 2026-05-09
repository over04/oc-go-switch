use regex::Regex;
use reqwest::Client;
use serde::de::DeserializeOwned;
use std::sync::atomic::{AtomicU64, Ordering};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ServerFnError {
    #[error("HTTP 错误: {0}")]
    Http(#[from] reqwest::Error),
    #[error("Server Function 返回错误: {status} {message}")]
    ServerError { status: u16, message: String },
    #[error("解析 $R 响应失败: {0}")]
    ParseError(String),
    #[error("Server Function 返回空响应")]
    EmptyResponse,
}

static INSTANCE: AtomicU64 = AtomicU64::new(0);

fn next_instance() -> String {
    let n = INSTANCE.fetch_add(1, Ordering::Relaxed);
    format!("server-fn:{}", n)
}

/// 调用 OpenCode SolidStart Server Function。
/// `hash` 为 64 位十六进制 server function ID。
/// `args` 为 JSON 可序列化的参数值（通常是元组或数组）。
/// 返回解析后的响应体。对于返回数组的函数，响应为 `$R[...]` 数组的第一个元素。
#[allow(dead_code)]
pub async fn call<T: DeserializeOwned>(
    client: &Client,
    hash: &str,
    args: &impl serde::Serialize,
) -> Result<T, ServerFnError> {
    let instance = next_instance();
    let body =
        serde_json::to_string(args).map_err(|e| ServerFnError::ParseError(e.to_string()))?;

    let resp = client
        .post("https://opencode.ai/_server")
        .header("X-Server-Id", hash)
        .header("X-Server-Instance", &instance)
        .header("Content-Type", "application/json")
        .body(body)
        .send()
        .await?;

    let status = resp.status();
    let text = resp.text().await?;

    if !status.is_success() {
        return Err(ServerFnError::ServerError {
            status: status.as_u16(),
            message: text,
        });
    }

    if text.is_empty() {
        return Err(ServerFnError::EmptyResponse);
    }

    parse_r_response(&text)
}

/// 调用无参数的 Server Function。
pub async fn call_no_args<T: DeserializeOwned>(
    client: &Client,
    hash: &str,
) -> Result<T, ServerFnError> {
    let instance = next_instance();

    let resp = client
        .post("https://opencode.ai/_server")
        .header("X-Server-Id", hash)
        .header("X-Server-Instance", &instance)
        .send()
        .await?;

    let status = resp.status();
    let text = resp.text().await?;

    if !status.is_success() {
        return Err(ServerFnError::ServerError {
            status: status.as_u16(),
            message: text,
        });
    }

    if text.is_empty() {
        return Err(ServerFnError::EmptyResponse);
    }

    parse_r_response(&text)
}

/// 解析 SolidStart `$R` 分块响应格式。
///
/// 响应使用类 JSON 格式，key 不带引号：
/// `$R[0]=[$R[1]={id:"wrk_...",name:"Default",slug:null}]`
///
/// 策略：
/// 1. 找到 `$R[0]=` 后跟的数据数组/对象
/// 2. 将无引号的 key 转为带引号的
/// 3. 作为 JSON 解析
fn parse_r_response<T: DeserializeOwned>(text: &str) -> Result<T, ServerFnError> {
    let re = Regex::new(r#"\$R\[0\]\s*=\s*"#)
        .map_err(|e: regex::Error| ServerFnError::ParseError(e.to_string()))?;

    if let Some(cap) = re.find(text) {
        let after = &text[cap.end()..];

        // 通过括号/大括号匹配提取 JSON 值
        if let Some(raw) = extract_json_value(after) {
            // SolidJS 格式使用无引号 key: `id:"value"` 而不是 `"id":"value"`
            let fixed = solidjs_to_json(&raw);
            return serde_json::from_str(&fixed).map_err(|e| {
                ServerFnError::ParseError(format!("JSON 解析错误: {e} | 输入: {fixed}"))
            });
        }
    }

    // 回退：尝试将整个响应作为 JSON 解析
    if let Ok(val) = serde_json::from_str::<T>(text) {
        return Ok(val);
    }

    Err(ServerFnError::ParseError(format!(
        "无法从响应中提取数据: {}...",
        &text[..text.len().min(200)]
    )))
}

/// 将 SolidJS 类 JSON 格式转为合法 JSON。
///
/// 转换：
/// 1. 剥离 `$R[N]=` 前缀
/// 2. 给无引号对象 key 加上引号 `key:` → `"key":`
fn solidjs_to_json(s: &str) -> String {
    let re_ref = Regex::new(r#"\$R\[\d+\]="#).unwrap();
    let cleaned = re_ref.replace_all(s, "").to_string();

    let re_key = Regex::new(r#"(?m)(^|[{,]\s*)([a-zA-Z_]\w*)\s*:"#).unwrap();
    re_key.replace_all(&cleaned, r#"$1"$2":"#).to_string()
}

/// 通过括号/大括号计数提取当前位置开始的 JSON 值。
fn extract_json_value(s: &str) -> Option<String> {
    let chars: Vec<char> = s.chars().collect();
    let first = chars.first()?;

    let (open, close) = match first {
        '[' => ('[', ']'),
        '{' => ('{', '}'),
        '"' => {
            let end_idx = s[1..].find('"').map(|i| i + 2)?;
            return Some(s[..end_idx].to_string());
        }
        _ => return None,
    };

    let mut depth = 0;
    let mut in_string = false;
    let mut escape = false;

    for (i, &c) in chars.iter().enumerate() {
        if escape {
            escape = false;
            continue;
        }
        if c == '\\' && in_string {
            escape = true;
            continue;
        }
        if c == '"' {
            in_string = !in_string;
            continue;
        }
        if in_string {
            continue;
        }
        if c == open {
            depth += 1;
        } else if c == close {
            depth -= 1;
            if depth == 0 {
                return Some(chars[..=i].iter().collect());
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_workspaces_response() {
        let text = r#";0x001111;((self.$R=self.$R||{})["server-fn:1"]=[],($R=>$R[0]=[$R[1]={id:"wrk_01KNRC73YQ7AHG7BPRXDBMTH2J",name:"Default",slug:null},$R[2]={id:"wrk_01KQW656KVG0B2HMG9K5DP0DMJ",name:"001",slug:null},$R[3]={id:"wrk_01KR570HB7SVVPHQ1R5QSGAVKQ",name:"002",slug:null}])($R["server-fn:1"]))"#;

        let result: Vec<serde_json::Value> = parse_r_response(text).unwrap();
        assert_eq!(result.len(), 3);
        assert_eq!(result[0]["name"], "Default");
        assert_eq!(result[1]["name"], "001");
        assert_eq!(result[2]["name"], "002");
    }

    #[test]
    fn test_solidjs_to_json() {
        let solid = r#"{id:"wrk_01",name:"Default",slug:null}"#;
        let json = solidjs_to_json(solid);
        assert!(json.contains("\"id\""));
        assert!(json.contains("\"name\""));
        assert!(json.contains("\"slug\""));
    }

    #[test]
    fn test_extract_json_array() {
        let s = r#"[1,2,3]rest"#;
        assert_eq!(extract_json_value(s).unwrap(), "[1,2,3]");
    }

    #[test]
    fn test_extract_json_object() {
        let s = r#"{"a":1,"b":[2,3]}xyz"#;
        assert_eq!(extract_json_value(s).unwrap(), r#"{"a":1,"b":[2,3]}"#);
    }
}
