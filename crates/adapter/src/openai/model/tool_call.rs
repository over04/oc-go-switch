use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct OpenAiMessageToolCall {
    pub id: String,
    #[serde(rename = "type")]
    pub call_type: OpenAiMessageToolCallType,
    pub function: OpenAiMessageToolCallFunction,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extra_content: Option<OpenAiMessageToolCallExtraContent>,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub enum OpenAiMessageToolCallType {
    #[serde(rename = "function")]
    Function,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct OpenAiMessageToolCallFunction {
    #[serde(deserialize_with = "crate::openai::model::stringified_json::deserialize")]
    pub arguments: String,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct OpenAiMessageToolCallExtraContent {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub google: Option<OpenAiMessageToolCallGoogleExtraContent>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct OpenAiMessageToolCallGoogleExtraContent {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub thought_signature: Option<String>,
}
