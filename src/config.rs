pub mod store;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub listen: String,
    pub accounts: Vec<AccountConfig>,
    #[serde(default = "default_refresh_interval")]
    pub refresh_interval_secs: u64,
    #[serde(default = "default_max_retries")]
    pub max_retries: usize,
    #[serde(default = "default_go")]
    pub go: GoConfig,
    #[serde(default)]
    pub image_filter: ImageFilterConfig,
    /// 管理 API 与协议入口共用 token。
    pub api_token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoConfig {
    pub base_url: String,
}

fn default_go() -> GoConfig {
    GoConfig {
        base_url: "https://api.opencode.ai/v1".to_string(),
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountConfig {
    pub name: String,
    pub auth: String,
    pub label: String,
}

/// 图片过滤行为。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FilterAction {
    /// 保留图片，不做处理（默认）。
    #[serde(rename = "pass_through")]
    PassThrough,
    /// 移除图片 block。
    Remove,
    /// 将图片 block 替换为文本。
    Replace,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ImageFilterConfig {
    #[serde(default)]
    pub models: Vec<ImageFilterModel>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageFilterModel {
    /// 模型 ID（精确匹配）。
    pub model: String,
    /// 对该模型的图片处理方式。
    pub action: FilterAction,
    /// 替换文本（仅 action 为 replace 时生效）。
    #[serde(default)]
    pub replacement: Option<String>,
}

fn default_refresh_interval() -> u64 {
    300
}

fn default_max_retries() -> usize {
    10
}

impl Config {
    pub fn from_yaml(content: &str) -> Result<Self, anyhow::Error> {
        let config: Config = serde_yaml::from_str(content)?;
        config.validate()?;
        Ok(config)
    }

    pub fn validate(&self) -> Result<(), anyhow::Error> {
        if self.max_retries > 100 {
            return Err(anyhow::anyhow!(
                "max_retries 不能超过 100，当前值为 {}",
                self.max_retries
            ));
        }
        validate_token("api_token", &self.api_token)?;
        Ok(())
    }
}

fn validate_token(name: &str, value: &str) -> Result<(), anyhow::Error> {
    if value.trim().is_empty() {
        return Err(anyhow::anyhow!("config.yaml 缺少有效的 {name}"));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn valid_yaml() -> String {
        r#"
listen: 127.0.0.1:8180
accounts: []
refresh_interval_secs: 300
max_retries: 8
go:
  base_url: https://opencode.ai/zen/go/v1
image_filter:
  models: []
api_token: admin-token
"#
        .to_string()
    }

    #[test]
    fn load_requires_api_token() {
        let cfg = Config::from_yaml(&valid_yaml()).unwrap();
        assert_eq!(cfg.api_token, "admin-token");
    }

    #[test]
    fn blank_token_is_invalid() {
        let yaml = valid_yaml().replace("admin-token", " ");
        let err = Config::from_yaml(&yaml).unwrap_err().to_string();
        assert!(err.contains("api_token"));
    }

    #[test]
    fn max_retries_limit_is_enforced() {
        let yaml = valid_yaml().replace("max_retries: 8", "max_retries: 101");
        let err = Config::from_yaml(&yaml).unwrap_err().to_string();
        assert!(err.contains("max_retries"));
    }
}
