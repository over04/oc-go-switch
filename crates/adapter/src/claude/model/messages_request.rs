use serde::{Deserialize, Serialize};

use crate::claude::model::{
    message::AnthropicMessage,
    request_field::{
        AnthropicCacheControl, AnthropicContainer, AnthropicContextManagement,
        AnthropicInferenceGeo, AnthropicMcpServer, AnthropicMetadata, AnthropicOutputConfig,
        AnthropicSpeed, AnthropicThinking, AnthropicTool, AnthropicToolChoice,
    },
    system::AnthropicSystemContent,
};

/// Anthropic messages 请求模型。
///
/// `@ai-sdk/anthropic` messages 请求模型。
#[derive(Debug, Deserialize, Serialize)]
pub struct AnthropicMessagesRequest {
    /// Anthropic 模型名。
    pub model: String,
    /// 对话消息列表。
    pub messages: Vec<AnthropicMessage>,
    /// 最大输出 token；extended thinking 场景允许省略。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u64>,
    /// 是否使用 SSE 流式响应。
    #[serde(default)]
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub stream: bool,
    /// 顶层 system 内容。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub system: Option<AnthropicSystemContent>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub top_k: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stop_sequences: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub thinking: Option<AnthropicThinking>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub output_config: Option<AnthropicOutputConfig>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub speed: Option<AnthropicSpeed>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inference_geo: Option<AnthropicInferenceGeo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cache_control: Option<AnthropicCacheControl>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<AnthropicMetadata>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mcp_servers: Option<Vec<AnthropicMcpServer>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub container: Option<AnthropicContainer>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub context_management: Option<AnthropicContextManagement>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<AnthropicTool>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<AnthropicToolChoice>,
}

impl AnthropicMessagesRequest {
    pub fn validate(&self) -> Result<(), &'static str> {
        if self.model.is_empty() {
            return Err("'model' 是必填字段");
        }
        if self.messages.is_empty() {
            return Err("'messages' 是必填字段，且不能为空数组");
        }
        Ok(())
    }
}
