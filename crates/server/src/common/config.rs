//! 配置命名空间。
//!
//! `Config` 只表达磁盘配置形状：`fixed` 是启动期配置，`runtime` 是热更新配置。
//! 请求路径读取 `ConfigRuntime`，配置写入由业务层服务统一处理。

pub mod account;
pub mod error;
pub mod filter_action;
pub mod fixed;
pub mod go;
pub mod image_filter;
pub mod image_filter_model;
pub mod runtime;
pub mod runtime_store;
pub mod store;

use serde::{Deserialize, Serialize};

use crate::common::config::{error::ConfigError, fixed::FixedConfig, runtime::RuntimeConfig};

/// 服务配置文件。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// 启动期配置，保存后需要重启进程生效。
    pub fixed: FixedConfig,
    /// 运行期配置，保存后立即影响新请求和后台任务。
    pub runtime: RuntimeConfig,
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
        self.fixed.validate()?;
        self.runtime.validate()
    }
}

#[cfg(test)]
mod tests {
    use crate::common::config::Config;

    fn valid_yaml() -> String {
        r#"
fixed:
  listen: 127.0.0.1:8180
runtime:
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
        assert_eq!(cfg.runtime.api_token, "admin-token");
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
