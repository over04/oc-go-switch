use serde::{Deserialize, Serialize};

use crate::common::config::{
    account::AccountConfig, error::ConfigError, go::GoConfig, image_filter::ImageFilterConfig,
};

/// 运行期配置。
///
/// 该配置保存后会立即用于新请求与后台任务。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeConfig {
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

impl RuntimeConfig {
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
        connect_timeout_secs: crate::common::config::go::default_timeout_secs(),
        request_timeout_secs: crate::common::config::go::default_timeout_secs(),
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
