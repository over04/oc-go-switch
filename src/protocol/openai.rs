use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// OpenAI /v1/chat/completions 请求体。
#[derive(Debug, Deserialize, Serialize)]
pub struct ChatCompletionRequest {
    pub model: String,
    pub messages: Vec<ChatMessage>,
    #[serde(default)]
    pub stream: bool,
    /// 所有未显式建模的字段（temperature, top_p, tools 等）完整透传。
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// 消息角色。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ChatRole {
    System,
    User,
    Assistant,
    Tool,
    Function,
}

/// 单条消息。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ChatMessage {
    pub role: ChatRole,
    pub content: ChatContent,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// content 字段：纯文本字符串 或 多模态 content part 数组。
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ChatContent {
    Text(String),
    Parts(Vec<ContentPart>),
}

/// 多模态 content part，按 type 字段区分。
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum ContentPart {
    #[serde(rename = "text")]
    Text { text: String },
    #[serde(rename = "image_url")]
    ImageUrl { image_url: ImageUrlObj },
    /// input_audio / file / 未来扩展类型 —— 透传。
    #[serde(untagged)]
    Other(Value),
}

/// 图片清晰度。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ImageDetail {
    Auto,
    Low,
    High,
}

/// image_url 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ImageUrlObj {
    pub url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub detail: Option<ImageDetail>,
}

impl ChatCompletionRequest {
    /// 反序列化后校验必填字段。
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
