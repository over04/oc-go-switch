use serde::{Deserialize, Serialize};

use crate::common::config::error::ConfigError;

/// 启动期配置。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FixedConfig {
    /// 服务监听地址，例如 `127.0.0.1:8180`。
    pub listen: String,
}

impl FixedConfig {
    pub fn validate(&self) -> Result<(), ConfigError> {
        self.listen
            .parse::<std::net::SocketAddr>()
            .map_err(|_| ConfigError::InvalidListen(self.listen.clone()))?;
        Ok(())
    }
}
