use std::{path::PathBuf, sync::Arc};

use thiserror::Error;
use tokio::io::AsyncWriteExt;

use crate::config::Config;

#[derive(Debug, Error)]
pub enum ConfigStoreError {
    #[error("读取配置文件失败: {0}")]
    Read(#[source] std::io::Error),
    #[error("解析配置文件失败: {0}")]
    Parse(#[source] anyhow::Error),
    #[error("序列化配置失败: {0}")]
    Serialize(#[source] serde_yaml::Error),
    #[error("打开配置文件失败: {0}")]
    Open(#[source] std::io::Error),
    #[error("写入配置文件失败: {0}")]
    Write(#[source] std::io::Error),
    #[error("刷新配置文件失败: {0}")]
    Flush(#[source] std::io::Error),
    #[error("同步配置文件失败: {0}")]
    Sync(#[source] std::io::Error),
}

#[derive(Debug, Clone)]
pub struct ConfigStore {
    path: Arc<PathBuf>,
}

impl ConfigStore {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self {
            path: Arc::new(path.into()),
        }
    }

    pub async fn load(&self) -> Result<Config, ConfigStoreError> {
        let content = tokio::fs::read_to_string(self.path.as_ref())
            .await
            .map_err(ConfigStoreError::Read)?;
        Config::from_yaml(&content).map_err(ConfigStoreError::Parse)
    }

    pub async fn save(&self, config: &Config) -> Result<(), ConfigStoreError> {
        config.validate().map_err(ConfigStoreError::Parse)?;
        let yaml = serde_yaml::to_string(config).map_err(ConfigStoreError::Serialize)?;
        let mut file = tokio::fs::OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(self.path.as_ref())
            .await
            .map_err(ConfigStoreError::Open)?;
        file.write_all(yaml.as_bytes())
            .await
            .map_err(ConfigStoreError::Write)?;
        file.flush().await.map_err(ConfigStoreError::Flush)?;
        file.sync_data().await.map_err(ConfigStoreError::Sync)?;
        Ok(())
    }
}
