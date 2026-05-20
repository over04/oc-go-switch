use std::time::Duration;

use crate::common::config::go::GoConfig;

/// 由当前运行时配置构建的 HTTP client 集合。
#[derive(Debug)]
pub struct ClientSet {
    pub proxy: reqwest::Client,
    pub short: reqwest::Client,
}

impl ClientSet {
    pub fn try_new(go: &GoConfig) -> Result<Self, reqwest::Error> {
        let connect_timeout = Duration::from_secs(go.connect_timeout_secs);
        let request_timeout = Duration::from_secs(go.request_timeout_secs);
        let proxy = reqwest::Client::builder()
            .connect_timeout(connect_timeout)
            .build()?;
        let short = reqwest::Client::builder()
            .connect_timeout(connect_timeout)
            .timeout(request_timeout)
            .build()?;
        Ok(Self { proxy, short })
    }
}
