use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Anthropic /v1/messages 请求体。
#[derive(Debug, Deserialize, Serialize)]
pub struct AnthropicMessagesRequest {
    pub model: String,
    pub messages: Vec<AnthropicMessage>,
    pub max_tokens: u64,
    #[serde(default)]
    pub stream: bool,
    /// 顶层 system 字段：字符串 或 文本 block 数组。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub system: Option<SystemContent>,
    /// 所有未显式建模的字段（temperature, top_p, top_k, thinking, tools 等）完整透传。
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// system 字段：纯文本 或 `[{type: "text", text: "..."}]`。
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum SystemContent {
    Text(String),
    Blocks(Vec<SystemTextBlock>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SystemBlockType {
    Text,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SystemTextBlock {
    #[serde(rename = "type")]
    pub block_type: SystemBlockType,
    pub text: String,
}

/// 消息角色。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AnthropicRole {
    User,
    Assistant,
}

/// 单条消息。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AnthropicMessage {
    pub role: AnthropicRole,
    pub content: AnthropicContent,
}

/// content 字段：纯文本字符串 或 content block 数组。
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum AnthropicContent {
    Text(String),
    Blocks(Vec<AnthropicContentBlock>),
}

/// 多模态 content block，按 type 字段区分。
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum AnthropicContentBlock {
    #[serde(rename = "text")]
    Text { text: String },
    #[serde(rename = "image")]
    Image { source: AnthropicImageSource },
    /// tool_use / tool_result / thinking / redacted_thinking / document / … 透传。
    #[serde(untagged)]
    Other(Value),
}

/// 图片来源：base64 编码 或 URL。
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum AnthropicImageSource {
    #[serde(rename = "base64")]
    Base64 {
        media_type: String,
        data: String,
    },
    #[serde(rename = "url")]
    Url {
        url: String,
    },
}

impl AnthropicMessagesRequest {
    /// 反序列化后校验必填字段。
    pub fn validate(&self) -> Result<(), &'static str> {
        if self.model.is_empty() {
            return Err("'model' 是必填字段");
        }
        if self.messages.is_empty() {
            return Err("'messages' 是必填字段，且不能为空数组");
        }
        if self.max_tokens == 0 {
            return Err("'max_tokens' 是必填字段");
        }
        Ok(())
    }
}
