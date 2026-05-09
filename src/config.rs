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

/// Image filter action.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FilterAction {
    /// Keep images as-is (default).
    #[serde(rename = "pass_through")]
    PassThrough,
    /// Remove image blocks entirely.
    Remove,
    /// Replace image blocks with text.
    Replace,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ImageFilterConfig {
    #[serde(default)]
    pub models: Vec<ImageFilterModel>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageFilterModel {
    /// Model ID (exact match).
    pub model: String,
    /// How to handle images for this model.
    pub action: FilterAction,
    /// Replacement text (only used when action is `replace`).
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
        Ok(config)
    }
}
