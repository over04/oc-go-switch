use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Anthropic /v1/messages request body.
#[derive(Debug, Deserialize, Serialize)]
pub struct AnthropicMessagesRequest {
    pub model: String,
    pub messages: Vec<AnthropicMessage>,
    pub max_tokens: u64,
    #[serde(default)]
    pub stream: bool,
    /// Top-level `system` field: string or array of text blocks.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub system: Option<SystemContent>,
    /// All fields not explicitly modeled (temperature, top_p, top_k, thinking, tools, etc.)
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// `system` field: plain string OR `[{type: "text", text: "..."}]`
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AnthropicRole {
    User,
    Assistant,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AnthropicMessage {
    pub role: AnthropicRole,
    pub content: AnthropicContent,
}

/// `content` field: plain text string OR array of content blocks.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum AnthropicContent {
    Text(String),
    Blocks(Vec<AnthropicContentBlock>),
}

/// Content block, discriminated by `type`.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum AnthropicContentBlock {
    #[serde(rename = "text")]
    Text { text: String },
    #[serde(rename = "image")]
    Image { source: AnthropicImageSource },
    /// tool_use / tool_result / thinking / redacted_thinking / document / …
    #[serde(untagged)]
    Other(Value),
}

/// Image source: base64-encoded or URL.
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
    /// Validate required fields after deserialization.
    pub fn validate(&self) -> Result<(), &'static str> {
        if self.model.is_empty() {
            return Err("'model' is required");
        }
        if self.messages.is_empty() {
            return Err("'messages' is required and must be a non-empty array");
        }
        if self.max_tokens == 0 {
            return Err("'max_tokens' is required");
        }
        Ok(())
    }
}
