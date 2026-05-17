use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::openai::model::{content::OpenAiChatContent, role::OpenAiChatRole};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OpenAiChatMessage {
    /// OpenAI 消息角色。
    pub role: OpenAiChatRole,
    /// 消息内容；assistant 携带 tool_calls 时可为空。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<OpenAiChatContent>,
    /// OpenAI 可选 name 字段。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 未建模协议扩展字段，例如 tool_calls、tool_call_id、refusal。
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}
