use crate::claude::model::{
    content::AnthropicContent, messages_request::AnthropicMessagesRequest, role::AnthropicRole,
    system::AnthropicSystemContent,
};

#[test]
fn omit_max_tokens_extended_thinking() -> Result<(), Box<dyn std::error::Error>> {
    let json = r#"{
        "model": "claude-sonnet-4-6",
        "messages": [{"role": "user", "content": "hello"}]
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
        "messages": [{"role": "user", "content": "hello"}]
    }"#;
    let req: AnthropicMessagesRequest = serde_json::from_str(json)?;
    assert_eq!(req.max_tokens, Some(1024));
    let out = serde_json::to_string(&req)?;
    assert!(out.contains("max_tokens"));
    Ok(())
}

#[test]
fn content_text_string() -> Result<(), Box<dyn std::error::Error>> {
    let json = r#"{
        "model": "claude-sonnet-4-6",
        "max_tokens": 100,
        "messages": [{"role": "user", "content": "simple text"}]
    }"#;
    let req: AnthropicMessagesRequest = serde_json::from_str(json)?;
    assert!(matches!(req.messages[0].content, AnthropicContent::Text(_)));
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
    assert!(matches!(
        req.messages[0].content,
        AnthropicContent::Blocks(_)
    ));
    let out = serde_json::to_string(&req)?;
    assert!(out.contains("tool_use"));
    Ok(())
}

#[test]
fn system_as_string() -> Result<(), Box<dyn std::error::Error>> {
    let json = r#"{
        "model": "claude-sonnet-4-6",
        "max_tokens": 100,
        "messages": [{"role": "user", "content": "hi"}],
        "system": "You are helpful."
    }"#;
    let req: AnthropicMessagesRequest = serde_json::from_str(json)?;
    assert!(matches!(req.system, Some(AnthropicSystemContent::Text(_))));
    Ok(())
}

#[test]
fn system_as_blocks() -> Result<(), Box<dyn std::error::Error>> {
    let json = r#"{
        "model": "claude-sonnet-4-6",
        "max_tokens": 100,
        "messages": [{"role": "user", "content": "hi"}],
        "system": [{"type": "text", "text": "You are helpful."}]
    }"#;
    let req: AnthropicMessagesRequest = serde_json::from_str(json)?;
    assert!(matches!(
        req.system,
        Some(AnthropicSystemContent::Blocks(_))
    ));
    Ok(())
}

#[test]
fn unknown_role_falls_back() -> Result<(), Box<dyn std::error::Error>> {
    let json = r#"{
        "model": "claude-sonnet-4-6",
        "max_tokens": 100,
        "messages": [{"role": "future_role", "content": "test"}]
    }"#;
    let req: AnthropicMessagesRequest = serde_json::from_str(json)?;
    assert_eq!(req.messages[0].role, AnthropicRole::Unknown);
    Ok(())
}

#[test]
fn unknown_system_block_kind_falls_back() -> Result<(), Box<dyn std::error::Error>> {
    let json = r#"{
        "model": "claude-sonnet-4-6",
        "max_tokens": 100,
        "messages": [{"role": "user", "content": "hi"}],
        "system": [{"type": "future_block", "text": "test"}]
    }"#;
    let req: AnthropicMessagesRequest = serde_json::from_str(json)?;
    assert!(matches!(
        req.system,
        Some(AnthropicSystemContent::Blocks(_))
    ));
    Ok(())
}

#[test]
fn unknown_image_source_type() -> Result<(), Box<dyn std::error::Error>> {
    let json = r#"{
        "model": "claude-sonnet-4-6",
        "max_tokens": 100,
        "messages": [{"role": "user", "content": [{"type": "image", "source": {"type": "s3", "bucket": "x", "key": "y"}}]}]
    }"#;
    let req: AnthropicMessagesRequest = serde_json::from_str(json)?;
    let out = serde_json::to_string(&req)?;
    assert!(out.contains("s3"));
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
    let json = r#"{"model": "claude-sonnet-4-6", "max_tokens": 100, "messages": [{"role": "user", "content": "hi"}], "temperature": 0.7, "thinking": {"type": "enabled", "budget_tokens": 500}}"#;
    let req: AnthropicMessagesRequest = serde_json::from_str(json)?;
    let out = serde_json::to_string(&req)?;
    assert!(out.contains("temperature"));
    assert!(out.contains("thinking"));
    Ok(())
}
