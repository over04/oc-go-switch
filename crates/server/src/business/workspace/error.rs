use thiserror::Error;

use crate::common::config::{error::ConfigError, store::ConfigStoreError};

#[derive(Debug, Error)]
pub enum PoolError {
    #[error("发现账户失败: {0}")]
    Discover(String),
    #[error("未发现工作区，请检查配置和账户授权。")]
    NoWorkspace,
    #[error("配置无效: {0}")]
    Config(#[from] ConfigError),
    #[error("保存配置失败: {0}")]
    ConfigStore(#[from] ConfigStoreError),
    #[error("构建 HTTP client 失败: {0}")]
    Client(#[from] reqwest::Error),
}
