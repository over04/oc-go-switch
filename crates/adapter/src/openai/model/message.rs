use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::openai::model::{
    assistant_content::OpenAiAssistantContent, content::OpenAiChatContent,
    tool_call::OpenAiMessageToolCall,
};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "role")]
pub enum OpenAiChatMessage {
    #[serde(rename = "system")]
    System {
        content: String,
        #[serde(flatten)]
        extra: HashMap<String, Value>,
    },
    #[serde(rename = "user")]
    User {
        content: OpenAiChatContent,
        #[serde(flatten)]
        extra: HashMap<String, Value>,
    },
    #[serde(rename = "assistant")]
    Assistant {
        #[serde(default, skip_serializing_if = "OpenAiAssistantContent::is_missing")]
        content: OpenAiAssistantContent,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        reasoning_content: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        tool_calls: Option<Vec<OpenAiMessageToolCall>>,
        #[serde(flatten)]
        extra: HashMap<String, Value>,
    },
    #[serde(rename = "tool")]
    Tool {
        content: String,
        tool_call_id: String,
        #[serde(flatten)]
        extra: HashMap<String, Value>,
    },
}
