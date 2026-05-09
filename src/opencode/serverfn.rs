use regex::Regex;
use reqwest::Client;
use serde::de::DeserializeOwned;
use std::sync::atomic::{AtomicU64, Ordering};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ServerFnError {
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),
    #[error("Server function returned error: {status} {message}")]
    ServerError { status: u16, message: String },
    #[error("Failed to parse $R response: {0}")]
    ParseError(String),
    #[error("Server function returned empty response")]
    EmptyResponse,
}

static INSTANCE: AtomicU64 = AtomicU64::new(0);

fn next_instance() -> String {
    let n = INSTANCE.fetch_add(1, Ordering::Relaxed);
    format!("server-fn:{}", n)
}

/// Call an OpenCode SolidStart server function.
///
/// `hash` is the 64-char SHA256 server function ID.
/// `args` is a JSON-serializable value (typically a tuple or array).
///
/// Returns the parsed response body. For array-returning functions (like
/// getWorkspaces which returns `[{id, name, slug}]`), the response is the
/// first element of the `$R[...]` array.
#[allow(dead_code)]
pub async fn call<T: DeserializeOwned>(
    client: &Client,
    hash: &str,
    args: &impl serde::Serialize,
) -> Result<T, ServerFnError> {
    let instance = next_instance();
    let body = serde_json::to_string(args).map_err(|e| ServerFnError::ParseError(e.to_string()))?;

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

/// Call a server function with no arguments.
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

/// Parse SolidStart `$R` chunked response format.
///
/// The response uses a JSON-LIKE format with unquoted keys:
/// `$R[0]=[$R[1]={id:"wrk_...",name:"Default",slug:null}]`
///
/// Strategy:
/// 1. Find `$R[0]=` followed by the data array/object
/// 2. Transform unquoted keys to quoted ones
/// 3. Parse as JSON
fn parse_r_response<T: DeserializeOwned>(text: &str) -> Result<T, ServerFnError> {
    let re =
        Regex::new(r#"\$R\[0\]\s*=\s*"#).map_err(|e: regex::Error| ServerFnError::ParseError(e.to_string()))?;

    if let Some(cap) = re.find(text) {
        let after = &text[cap.end()..];

        // Extract the JSON value by bracket/brace matching
        if let Some(raw) = extract_json_value(after) {
            // SolidJS format uses unquoted keys: `id:"value"` instead of `"id":"value"`
            let fixed = solidjs_to_json(&raw);
            return serde_json::from_str(&fixed)
                .map_err(|e| ServerFnError::ParseError(format!("JSON parse error: {e} | input: {fixed}")));
        }
    }

    // Fallback: try parsing the entire response as JSON
    if let Ok(val) = serde_json::from_str::<T>(text) {
        return Ok(val);
    }

    Err(ServerFnError::ParseError(format!(
        "Cannot extract data from response: {}...",
        &text[..text.len().min(200)]
    )))
}

/// Convert SolidJS JSON-like format to valid JSON.
///
/// Transformations:
/// 1. Strip `$R[N]=` prefixes from array elements
/// 2. Quote unquoted object keys `key:` → `"key":`
fn solidjs_to_json(s: &str) -> String {
    // First: strip $R[N]= prefixes
    let re_ref = Regex::new(r#"\$R\[\d+\]="#).unwrap();
    let cleaned = re_ref.replace_all(s, "").to_string();

    // Second: quote unquoted keys (word char sequence followed by colon)
    let re_key = Regex::new(r#"(?m)(^|[{,]\s*)([a-zA-Z_]\w*)\s*:"#).unwrap();
    re_key.replace_all(&cleaned, r#"$1"$2":"#).to_string()
}

/// Extract a JSON value starting at the current position by counting brackets/braces.
fn extract_json_value(s: &str) -> Option<String> {
    let chars: Vec<char> = s.chars().collect();
    let first = chars.first()?;

    let (open, close) = match first {
        '[' => ('[', ']'),
        '{' => ('{', '}'),
        '"' => {
            // String value — find the closing unescaped quote
            let _end = chars[1..]
                .iter()
                .position(|&c| c == '"' && chars.get(chars.len().saturating_sub(1)) != Some(&'\\'));
            // Simplified string extraction
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
        // SolidJS format: unquoted keys
        let text = r#";0x00000111;((self.$R=self.$R||{})["server-fn:1"]=[],($R=>$R[0]=[$R[1]={id:"wrk_01KNRC73YQ7AHG7BPRXDBMTH2J",name:"Default",slug:null},$R[2]={id:"wrk_01KQW656KVG0B2HMG9K5DP0DMJ",name:"001",slug:null},$R[3]={id:"wrk_01KR570HB7SVVPHQ1R5QSGAVKQ",name:"002",slug:null}])($R["server-fn:1"]))"#;

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
