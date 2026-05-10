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
    Developer,
    #[serde(other)]
    Unknown,
}

/// 单条消息。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ChatMessage {
    pub role: ChatRole,
    /// OpenAI 协议中 assistant 消息带 tool_calls 时 content 可为 null。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<ChatContent>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 透传未显式建模的字段（tool_calls, tool_call_id, refusal 等）。
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// content 字段：纯文本字符串 或 多模态 content part 数组。
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ChatContent {
    Text(String),
    Parts(Vec<ContentPart>),
}

/// 多模态 content part，按 type 字段区分。
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum ContentPart {
    #[serde(rename = "text")]
    Text {
        text: String,
        #[serde(flatten)]
        extra: HashMap<String, Value>,
    },
    #[serde(rename = "image_url")]
    ImageUrl {
        image_url: ImageUrlObj,
        #[serde(flatten)]
        extra: HashMap<String, Value>,
    },
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
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn assistant_with_null_content_and_tool_calls() {
        let json = r#"{
            "model": "gpt-4",
            "messages": [
                {"role": "user", "content": "hello"},
                {"role": "assistant", "content": null, "tool_calls": [{"id": "1", "type": "function", "function": {"name": "test", "arguments": "{}"}}]}
            ]
        }"#;
        let req: ChatCompletionRequest = serde_json::from_str(json).unwrap();
        assert_eq!(req.messages[1].content, None);
        // tool_calls 应透传到 extra
        let extra = &req.messages[1].extra;
        assert!(extra.contains_key("tool_calls"));
        // 序列化回去，content 字段应被跳过，tool_calls 应保留
        let out = serde_json::to_string(&req).unwrap();
        assert!(out.contains("tool_calls"));
    }

    #[test]
    fn tool_message_with_content() {
        let json = r#"{
            "model": "gpt-4",
            "messages": [
                {"role": "user", "content": "hello"},
                {"role": "assistant", "content": null, "tool_calls": [{"id": "1", "type": "function", "function": {"name": "test", "arguments": "{}"}}]},
                {"role": "tool", "content": "result", "tool_call_id": "1"}
            ]
        }"#;
        let req: ChatCompletionRequest = serde_json::from_str(json).unwrap();
        assert_eq!(
            req.messages[2].content,
            Some(ChatContent::Text("result".into()))
        );
        assert_eq!(req.messages[2].extra.get("tool_call_id").unwrap(), "1");
    }

    #[test]
    fn developer_role() {
        let json = r#"{"model": "gpt-4", "messages": [{"role": "developer", "content": "You are helpful."}]}"#;
        let req: ChatCompletionRequest = serde_json::from_str(json).unwrap();
        assert_eq!(req.messages[0].role, ChatRole::Developer);
    }

    #[test]
    fn unknown_role_falls_back() {
        let json = r#"{"model": "gpt-4", "messages": [{"role": "future_role", "content": "test"}]}"#;
        let req: ChatCompletionRequest = serde_json::from_str(json).unwrap();
        assert_eq!(req.messages[0].role, ChatRole::Unknown);
    }

    #[test]
    fn content_array() {
        let json = r#"{
            "model": "gpt-4",
            "messages": [{"role": "user", "content": [{"type": "text", "text": "hello"}, {"type": "image_url", "image_url": {"url": "https://example.com/img.png"}}]}]
        }"#;
        let req: ChatCompletionRequest = serde_json::from_str(json).unwrap();
        assert!(matches!(req.messages[0].content, Some(ChatContent::Parts(_))));
    }

    #[test]
    fn content_text_string() {
        let json = r#"{"model": "gpt-4", "messages": [{"role": "user", "content": "simple text"}]}"#;
        let req: ChatCompletionRequest = serde_json::from_str(json).unwrap();
        assert_eq!(
            req.messages[0].content,
            Some(ChatContent::Text("simple text".into()))
        );
    }

    #[test]
    fn roundtrip_preserves_extra_fields() {
        let json = r#"{"model": "gpt-4", "messages": [{"role": "user", "content": "hi"}], "temperature": 0.7, "top_p": 0.9}"#;
        let req: ChatCompletionRequest = serde_json::from_str(json).unwrap();
        let out = serde_json::to_string(&req).unwrap();
        assert!(out.contains("temperature"));
        assert!(out.contains("top_p"));
    }
}
