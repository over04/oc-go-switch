use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::openai::model::message::OpenAiChatMessage;

/// OpenAI chat completions 请求模型。
///
/// 未显式建模的字段保存在 `extra`，用于透传 tools、temperature 等协议扩展字段。
#[derive(Debug, Deserialize, Serialize)]
pub struct OpenAiChatCompletionRequest {
    pub model: String,
    pub messages: Vec<OpenAiChatMessage>,
    #[serde(default)]
    pub stream: bool,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

impl OpenAiChatCompletionRequest {
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
