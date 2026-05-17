//! 配置命名空间。
//!
//! 该层只描述进程启动与运行时可修改的配置形状；业务层通过配置快照读取，
//! 文件读写由 `store` 子模块负责，避免请求路径持有配置文件锁。

pub mod account;
pub mod error;
pub mod filter_action;
pub mod go;
pub mod image_filter;
pub mod image_filter_model;
pub mod store;

use serde::{Deserialize, Serialize};

use crate::common::config::{
    account::AccountConfig, error::ConfigError, go::GoConfig, image_filter::ImageFilterConfig,
};

/// 服务运行配置。
///
/// `api_token` 是管理 API、OpenAI 协议入口和 Anthropic 协议入口共同使用的强制鉴权值。
/// 配置加载后立即校验，避免缺失 token 的进程继续启动。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// 服务监听地址，例如 `127.0.0.1:8180`。
    pub listen: String,
    /// OpenCode 账户列表，一个账户可发现多个工作区。
    pub accounts: Vec<AccountConfig>,
    /// 后台主动刷新工作区状态的间隔，单位秒；0 表示关闭后台刷新。
    #[serde(default = "default_refresh_interval")]
    pub refresh_interval_secs: u64,
    /// 单次代理请求最多切换工作区重试的次数。
    #[serde(default = "default_max_retries")]
    pub max_retries: usize,
    /// Go API 上游配置。
    #[serde(default = "default_go")]
    pub go: GoConfig,
    /// 图片过滤规则，按模型精确匹配。
    #[serde(default)]
    pub image_filter: ImageFilterConfig,
    /// 管理 API 与协议入口共用 token，空字符串属于配置错误。
    pub api_token: String,
}

impl Config {
    /// 从 YAML 文本解析配置，并执行启动级校验。
    pub fn from_yaml(content: &str) -> Result<Self, ConfigError> {
        let config: Config = serde_yaml::from_str(content)?;
        config.validate()?;
        Ok(config)
    }

    /// 校验影响进程安全性和稳定性的配置项。
    pub fn validate(&self) -> Result<(), ConfigError> {
        if self.max_retries > 100 {
            return Err(ConfigError::MaxRetriesTooLarge(self.max_retries));
        }
        validate_token("api_token", &self.api_token)?;
        Ok(())
    }
}

fn default_go() -> GoConfig {
    GoConfig {
        base_url: "https://api.opencode.ai/v1".to_string(),
        connect_timeout_secs: go::default_timeout_secs(),
        request_timeout_secs: go::default_timeout_secs(),
    }
}

fn default_refresh_interval() -> u64 {
    300
}

fn default_max_retries() -> usize {
    10
}

fn validate_token(name: &'static str, value: &str) -> Result<(), ConfigError> {
    if value.trim().is_empty() {
        return Err(ConfigError::MissingToken(name));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::common::config::Config;

    fn valid_yaml() -> String {
        r#"
listen: 127.0.0.1:8180
accounts: []
refresh_interval_secs: 300
max_retries: 8
go:
  base_url: https://opencode.ai/zen/go/v1
  connect_timeout_secs: 90
  request_timeout_secs: 90
image_filter:
  models: []
api_token: admin-token
"#
        .to_string()
    }

    #[test]
    fn load_requires_api_token() -> Result<(), Box<dyn std::error::Error>> {
        let cfg = Config::from_yaml(&valid_yaml())?;
        assert_eq!(cfg.api_token, "admin-token");
        Ok(())
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
