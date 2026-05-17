use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::claude::model::{message::AnthropicMessage, system::AnthropicSystemContent};

/// Anthropic messages 请求模型。
///
/// `max_tokens` 在 extended thinking 场景可省略，开放字段通过 `extra` 透传。
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
    pub stream: bool,
    /// 顶层 system 内容。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub system: Option<AnthropicSystemContent>,
    /// 未显式建模的 Anthropic 协议扩展字段。
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
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
