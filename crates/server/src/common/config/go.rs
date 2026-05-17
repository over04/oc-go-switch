use serde::{Deserialize, Serialize};

/// OpenCode Go 上游配置。
///
/// 协议路径由代理层按 OpenAI/Anthropic 入口拼接。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoConfig {
    /// OpenCode Go 兼容 API 的基础地址。
    pub base_url: String,
    /// 建立 TCP/TLS 连接的超时时间，单位秒。
    #[serde(default = "default_timeout_secs")]
    pub connect_timeout_secs: u64,
    /// 短请求总超时时间，单位秒。
    #[serde(default = "default_timeout_secs")]
    pub request_timeout_secs: u64,
}

pub fn default_timeout_secs() -> u64 {
    90
}
