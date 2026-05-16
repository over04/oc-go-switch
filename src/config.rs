use serde::{Deserialize, Serialize};
use std::path::Path;

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
    /// API token for management endpoints. If unset, management API is disabled.
    #[serde(default)]
    pub api_token: Option<String>,
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
    pub fn load(path: impl AsRef<Path>) -> Result<Self, anyhow::Error> {
        let content = std::fs::read_to_string(path)?;
        let config: Config = serde_yaml::from_str(&content)?;
        if config.max_retries > 100 {
            return Err(anyhow::anyhow!(
                "max_retries 不能超过 100，当前值为 {}",
                config.max_retries
            ));
        }
        if config.api_token.is_none() {
            return Err(anyhow::anyhow!(
                "config.yaml 缺少 api_token，管理 API 将不可用。请在 config.yaml 中设置 api_token"
            ));
        }
        Ok(config)
    }
}