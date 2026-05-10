use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Anthropic /v1/messages 请求体。
#[derive(Debug, Deserialize, Serialize)]
pub struct AnthropicMessagesRequest {
    pub model: String,
    pub messages: Vec<AnthropicMessage>,
    /// Anthropic 协议中启用 extended thinking 时可省略 max_tokens。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u64>,
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
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SystemTextBlock {
    #[serde(rename = "type")]
    pub block_type: SystemBlockType,
    pub text: String,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// 消息角色。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AnthropicRole {
    User,
    Assistant,
    #[serde(other)]
    Unknown,
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
    Text {
        text: String,
        #[serde(flatten)]
        extra: HashMap<String, Value>,
    },
    #[serde(rename = "image")]
    Image {
        source: AnthropicImageSource,
        #[serde(flatten)]
        extra: HashMap<String, Value>,
    },
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
        #[serde(flatten)]
        extra: HashMap<String, Value>,
    },
    #[serde(rename = "url")]
    Url {
        url: String,
        #[serde(flatten)]
        extra: HashMap<String, Value>,
    },
    /// 未来新增的图片来源类型透传。
    #[serde(untagged)]
    Other(Value),
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
        // max_tokens 不再强制校验：Anthropic 启用 extended thinking 时可省略。
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn omit_max_tokens_extended_thinking() {
        let json = r#"{
            "model": "claude-sonnet-4-6",
            "messages": [{"role": "user", "content": "hello"}]
        }"#;
        let req: AnthropicMessagesRequest = serde_json::from_str(json).unwrap();
        assert_eq!(req.max_tokens, None);
        // 序列化时应跳过 max_tokens，不输出 null
        let out = serde_json::to_string(&req).unwrap();
        assert!(!out.contains("max_tokens"));
    }

    #[test]
    fn with_max_tokens() {
        let json = r#"{
            "model": "claude-sonnet-4-6",
            "max_tokens": 1024,
            "messages": [{"role": "user", "content": "hello"}]
        }"#;
        let req: AnthropicMessagesRequest = serde_json::from_str(json).unwrap();
        assert_eq!(req.max_tokens, Some(1024));
        let out = serde_json::to_string(&req).unwrap();
        assert!(out.contains("max_tokens"));
    }

    #[test]
    fn content_text_string() {
        let json = r#"{
            "model": "claude-sonnet-4-6",
            "max_tokens": 100,
            "messages": [{"role": "user", "content": "simple text"}]
        }"#;
        let req: AnthropicMessagesRequest = serde_json::from_str(json).unwrap();
        assert!(matches!(req.messages[0].content, AnthropicContent::Text(_)));
    }

    #[test]
    fn content_blocks_with_tool_use() {
        let json = r#"{
            "model": "claude-sonnet-4-6",
            "max_tokens": 100,
            "messages": [{
                "role": "assistant",
                "content": [{"type": "text", "text": "Let me check"}, {"type": "tool_use", "id": "1", "name": "search", "input": {"q": "test"}}]
            }]
        }"#;
        let req: AnthropicMessagesRequest = serde_json::from_str(json).unwrap();
        assert!(matches!(
            req.messages[0].content,
            AnthropicContent::Blocks(_)
        ));
        // 序列化回去保留 tool_use
        let out = serde_json::to_string(&req).unwrap();
        assert!(out.contains("tool_use"));
    }

    #[test]
    fn system_as_string() {
        let json = r#"{
            "model": "claude-sonnet-4-6",
            "max_tokens": 100,
            "messages": [{"role": "user", "content": "hi"}],
            "system": "You are helpful."
        }"#;
        let req: AnthropicMessagesRequest = serde_json::from_str(json).unwrap();
        assert!(matches!(req.system, Some(SystemContent::Text(_))));
    }

    #[test]
    fn system_as_blocks() {
        let json = r#"{
            "model": "claude-sonnet-4-6",
            "max_tokens": 100,
            "messages": [{"role": "user", "content": "hi"}],
            "system": [{"type": "text", "text": "You are helpful."}]
        }"#;
        let req: AnthropicMessagesRequest = serde_json::from_str(json).unwrap();
        assert!(matches!(req.system, Some(SystemContent::Blocks(_))));
    }

    #[test]
    fn unknown_role_falls_back() {
        let json = r#"{
            "model": "claude-sonnet-4-6",
            "max_tokens": 100,
            "messages": [{"role": "future_role", "content": "test"}]
        }"#;
        let req: AnthropicMessagesRequest = serde_json::from_str(json).unwrap();
        assert_eq!(req.messages[0].role, AnthropicRole::Unknown);
    }

    #[test]
    fn unknown_system_block_type_falls_back() {
        let json = r#"{
            "model": "claude-sonnet-4-6",
            "max_tokens": 100,
            "messages": [{"role": "user", "content": "hi"}],
            "system": [{"type": "future_block", "text": "test"}]
        }"#;
        let req: AnthropicMessagesRequest = serde_json::from_str(json).unwrap();
        // 不应 panic
        assert!(matches!(req.system, Some(SystemContent::Blocks(_))));
    }

    #[test]
    fn unknown_image_source_type() {
        let json = r#"{
            "model": "claude-sonnet-4-6",
            "max_tokens": 100,
            "messages": [{"role": "user", "content": [{"type": "image", "source": {"type": "s3", "bucket": "x", "key": "y"}}]}]
        }"#;
        let req: AnthropicMessagesRequest = serde_json::from_str(json).unwrap();
        // 不应 panic，s3 类型应被 Other(Value) 捕获
        let out = serde_json::to_string(&req).unwrap();
        assert!(out.contains("s3"));
    }

    #[test]
    fn content_block_cache_control_passthrough() {
        let json = r#"{
            "model": "claude-sonnet-4-6",
            "max_tokens": 100,
            "messages": [{"role": "user", "content": [{"type": "text", "text": "hello", "cache_control": {"type": "ephemeral"}}]}]
        }"#;
        let req: AnthropicMessagesRequest = serde_json::from_str(json).unwrap();
        let out = serde_json::to_string(&req).unwrap();
        assert!(out.contains("cache_control"));
    }

    #[test]
    fn roundtrip_extra_fields() {
        let json = r#"{"model": "claude-sonnet-4-6", "max_tokens": 100, "messages": [{"role": "user", "content": "hi"}], "temperature": 0.7, "thinking": {"type": "enabled", "budget_tokens": 500}}"#;
        let req: AnthropicMessagesRequest = serde_json::from_str(json).unwrap();
        let out = serde_json::to_string(&req).unwrap();
        assert!(out.contains("temperature"));
        assert!(out.contains("thinking"));
    }
}
