use crate::openai::model::{
    completion_request::OpenAiChatCompletionRequest, content::OpenAiChatContent,
    role::OpenAiChatRole,
};

#[test]
fn assistant_with_null_content_and_tool_calls() -> Result<(), Box<dyn std::error::Error>> {
    let json = r#"{
        "model": "gpt-4",
        "messages": [
            {"role": "user", "content": "hello"},
            {"role": "assistant", "content": null, "tool_calls": [{"id": "1", "type": "function", "function": {"name": "test", "arguments": "{}"}}]}
        ]
    }"#;
    let req: OpenAiChatCompletionRequest = serde_json::from_str(json)?;
    assert_eq!(req.messages[1].content, None);
    assert!(req.messages[1].extra.contains_key("tool_calls"));
    let out = serde_json::to_string(&req)?;
    assert!(out.contains("tool_calls"));
    Ok(())
}

#[test]
fn tool_message_with_content() -> Result<(), Box<dyn std::error::Error>> {
    let json = r#"{
        "model": "gpt-4",
        "messages": [
            {"role": "user", "content": "hello"},
            {"role": "assistant", "content": null, "tool_calls": [{"id": "1", "type": "function", "function": {"name": "test", "arguments": "{}"}}]},
            {"role": "tool", "content": "result", "tool_call_id": "1"}
        ]
    }"#;
    let req: OpenAiChatCompletionRequest = serde_json::from_str(json)?;
    assert_eq!(
        req.messages[2].content,
        Some(OpenAiChatContent::Text("result".into()))
    );
    assert_eq!(
        req.messages[2]
            .extra
            .get("tool_call_id")
            .ok_or("missing tool_call_id")?,
        "1"
    );
    Ok(())
}

#[test]
fn developer_role() -> Result<(), Box<dyn std::error::Error>> {
    let json =
        r#"{"model": "gpt-4", "messages": [{"role": "developer", "content": "You are helpful."}]}"#;
    let req: OpenAiChatCompletionRequest = serde_json::from_str(json)?;
    assert_eq!(req.messages[0].role, OpenAiChatRole::Developer);
    Ok(())
}

#[test]
fn unknown_role_falls_back() -> Result<(), Box<dyn std::error::Error>> {
    let json = r#"{"model": "gpt-4", "messages": [{"role": "future_role", "content": "test"}]}"#;
    let req: OpenAiChatCompletionRequest = serde_json::from_str(json)?;
    assert_eq!(req.messages[0].role, OpenAiChatRole::Unknown);
    Ok(())
}

#[test]
fn content_array() -> Result<(), Box<dyn std::error::Error>> {
    let json = r#"{
        "model": "gpt-4",
        "messages": [{"role": "user", "content": [{"type": "text", "text": "hello"}, {"type": "image_url", "image_url": {"url": "https://example.com/img.png"}}]}]
    }"#;
    let req: OpenAiChatCompletionRequest = serde_json::from_str(json)?;
    assert!(matches!(
        req.messages[0].content,
        Some(OpenAiChatContent::Parts(_))
    ));
    Ok(())
}

#[test]
fn content_text_string() -> Result<(), Box<dyn std::error::Error>> {
    let json = r#"{"model": "gpt-4", "messages": [{"role": "user", "content": "simple text"}]}"#;
    let req: OpenAiChatCompletionRequest = serde_json::from_str(json)?;
    assert_eq!(
        req.messages[0].content,
        Some(OpenAiChatContent::Text("simple text".into()))
    );
    Ok(())
}

#[test]
fn roundtrip_preserves_extra_fields() -> Result<(), Box<dyn std::error::Error>> {
    let json = r#"{"model": "gpt-4", "messages": [{"role": "user", "content": "hi"}], "temperature": 0.7, "top_p": 0.9}"#;
    let req: OpenAiChatCompletionRequest = serde_json::from_str(json)?;
    let out = serde_json::to_string(&req)?;
    assert!(out.contains("temperature"));
    assert!(out.contains("top_p"));
    Ok(())
}
