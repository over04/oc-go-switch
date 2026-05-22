use crate::claude::model::{
    block::AnthropicContentBlock, message::AnthropicMessage,
    messages_request::AnthropicMessagesRequest,
};

#[test]
fn omit_max_tokens_extended_thinking() -> Result<(), Box<dyn std::error::Error>> {
    let json = r#"{
        "model": "claude-sonnet-4-6",
        "messages": [{"role": "user", "content": [{"type": "text", "text": "hello"}]}]
    }"#;
    let req: AnthropicMessagesRequest = serde_json::from_str(json)?;
    assert_eq!(req.max_tokens, None);
    let out = serde_json::to_string(&req)?;
    assert!(!out.contains("max_tokens"));
    Ok(())
}

#[test]
fn with_max_tokens() -> Result<(), Box<dyn std::error::Error>> {
    let json = r#"{
        "model": "claude-sonnet-4-6",
        "max_tokens": 1024,
        "messages": [{"role": "user", "content": [{"type": "text", "text": "hello"}]}]
    }"#;
    let req: AnthropicMessagesRequest = serde_json::from_str(json)?;
    assert_eq!(req.max_tokens, Some(1024));
    let out = serde_json::to_string(&req)?;
    assert!(out.contains("max_tokens"));
    Ok(())
}

#[test]
fn content_text_block() -> Result<(), Box<dyn std::error::Error>> {
    let json = r#"{
        "model": "claude-sonnet-4-6",
        "max_tokens": 100,
        "messages": [{"role": "user", "content": [{"type": "text", "text": "simple text"}]}]
    }"#;
    let req: AnthropicMessagesRequest = serde_json::from_str(json)?;
    let AnthropicMessage::User { content } = &req.messages[0] else {
        panic!("expected user message");
    };
    assert!(matches!(
        content.blocks[0],
        AnthropicContentBlock::Text { .. }
    ));
    Ok(())
}

#[test]
fn content_blocks_with_tool_use() -> Result<(), Box<dyn std::error::Error>> {
    let json = r#"{
        "model": "claude-sonnet-4-6",
        "max_tokens": 100,
        "messages": [{
            "role": "assistant",
            "content": [{"type": "text", "text": "Let me check"}, {"type": "tool_use", "id": "1", "name": "search", "input": {"q": "test"}}]
        }]
    }"#;
    let req: AnthropicMessagesRequest = serde_json::from_str(json)?;
    let AnthropicMessage::Assistant { content } = &req.messages[0] else {
        panic!("expected assistant message");
    };
    assert!(matches!(
        content.blocks[1],
        AnthropicContentBlock::ToolUse { .. }
    ));
    let out = serde_json::to_string(&req)?;
    assert!(out.contains("tool_use"));
    Ok(())
}

#[test]
fn system_as_text_blocks() -> Result<(), Box<dyn std::error::Error>> {
    let json = r#"{
        "model": "claude-sonnet-4-6",
        "max_tokens": 100,
        "messages": [{"role": "user", "content": [{"type": "text", "text": "hi"}]}],
        "system": [{"type": "text", "text": "You are helpful."}]
    }"#;
    let req: AnthropicMessagesRequest = serde_json::from_str(json)?;
    assert!(req.system.is_some());
    Ok(())
}

#[test]
fn content_block_cache_control_passthrough() -> Result<(), Box<dyn std::error::Error>> {
    let json = r#"{
        "model": "claude-sonnet-4-6",
        "max_tokens": 100,
        "messages": [{"role": "user", "content": [{"type": "text", "text": "hello", "cache_control": {"type": "ephemeral"}}]}]
    }"#;
    let req: AnthropicMessagesRequest = serde_json::from_str(json)?;
    let out = serde_json::to_string(&req)?;
    assert!(out.contains("cache_control"));
    Ok(())
}

#[test]
fn roundtrip_extra_fields() -> Result<(), Box<dyn std::error::Error>> {
    let json = r#"{"model": "claude-sonnet-4-6", "max_tokens": 100, "messages": [{"role": "user", "content": [{"type": "text", "text": "hi"}]}], "temperature": 0.7, "thinking": {"type": "enabled", "budget_tokens": 500}}"#;
    let req: AnthropicMessagesRequest = serde_json::from_str(json)?;
    let out = serde_json::to_string(&req)?;
    assert!(out.contains("temperature"));
    assert!(out.contains("thinking"));
    Ok(())
}

#[test]
fn mcp_server_nullish_fields_match_ai_sdk() -> Result<(), Box<dyn std::error::Error>> {
    let json = r#"{
        "model": "claude-sonnet-4-6",
        "max_tokens": 100,
        "messages": [{"role": "user", "content": [{"type": "text", "text": "hi"}]}],
        "mcp_servers": [{
            "type": "url",
            "name": "docs",
            "url": "https://example.com/mcp",
            "authorization_token": null,
            "tool_configuration": {
                "allowed_tools": null,
                "enabled": null
            }
        }]
    }"#;
    let req: AnthropicMessagesRequest = serde_json::from_str(json)?;
    let out = serde_json::to_string(&req)?;
    assert!(out.contains(r#""authorization_token":null"#));
    assert!(out.contains(r#""allowed_tools":null"#));
    assert!(out.contains(r#""enabled":null"#));
    Ok(())
}

#[test]
fn ai_sdk_anthropic_content_blocks_roundtrip() -> Result<(), Box<dyn std::error::Error>> {
    let json = r#"{
        "model": "claude-sonnet-4-6",
        "max_tokens": 100,
        "system": [{"type": "text", "text": "You are helpful.", "cache_control": {"type": "ephemeral"}}],
        "messages": [
            {
                "role": "user",
                "content": [
                    {"type": "document", "source": {"type": "text", "media_type": "text/plain", "data": "doc"}, "title": "doc.txt", "citations": {"enabled": true}},
                    {"type": "tool_result", "tool_use_id": "toolu_1", "content": [{"type": "text", "text": "result"}]}
                ]
            },
            {
                "role": "assistant",
                "content": [
                    {"type": "thinking", "thinking": "internal", "signature": "sig"},
                    {"type": "server_tool_use", "id": "srvu_1", "name": "web_search", "input": {"query": "rust"}},
                    {"type": "tool_use", "id": "toolu_1", "name": "lookup", "input": {"q": "rust"}, "caller": {"type": "direct"}}
                ]
            }
        ],
        "thinking": {"type": "enabled", "budget_tokens": 500},
        "context_management": {"edits": [{"type": "clear_thinking_20251015"}]}
    }"#;
    let req: AnthropicMessagesRequest = serde_json::from_str(json)?;
    let AnthropicMessage::User {
        content: user_content,
    } = &req.messages[0]
    else {
        panic!("expected user message");
    };
    let user_blocks = &user_content.blocks;
    assert!(matches!(
        user_blocks[0],
        AnthropicContentBlock::Document { .. }
    ));
    assert!(matches!(
        user_blocks[1],
        AnthropicContentBlock::ToolResult { .. }
    ));
    let AnthropicMessage::Assistant {
        content: assistant_content,
    } = &req.messages[1]
    else {
        panic!("expected assistant message");
    };
    let assistant_blocks = &assistant_content.blocks;
    assert!(matches!(
        assistant_blocks[0],
        AnthropicContentBlock::Thinking { .. }
    ));
    assert!(matches!(
        assistant_blocks[1],
        AnthropicContentBlock::ServerToolUse { .. }
    ));
    assert!(matches!(
        assistant_blocks[2],
        AnthropicContentBlock::ToolUse { .. }
    ));
    let out = serde_json::to_string(&req)?;
    assert!(out.contains("document"));
    assert!(out.contains("tool_result"));
    assert!(out.contains("server_tool_use"));
    assert!(out.contains("thinking"));
    assert!(out.contains("context_management"));
    assert!(out.contains(r#""caller":{"type":"direct"}"#));
    Ok(())
}

#[test]
fn false_stream_is_omitted_like_ai_sdk() -> Result<(), Box<dyn std::error::Error>> {
    let json = r#"{
        "model": "claude-sonnet-4-6",
        "max_tokens": 100,
        "messages": [{"role": "user", "content": [{"type": "text", "text": "hi"}]}]
    }"#;
    let req: AnthropicMessagesRequest = serde_json::from_str(json)?;
    let out = serde_json::to_string(&req)?;
    assert!(!out.contains("stream"));
    Ok(())
}
