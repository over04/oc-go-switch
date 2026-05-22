use crate::openai::model::{
    assistant_content::OpenAiAssistantContent, completion_request::OpenAiChatCompletionRequest,
    content::OpenAiChatContent, message::OpenAiChatMessage, part::OpenAiContentPart,
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
    let OpenAiChatMessage::Assistant {
        content,
        tool_calls,
        ..
    } = &req.messages[1]
    else {
        return Err("expected assistant message".into());
    };
    assert_eq!(content, &OpenAiAssistantContent::Null);
    assert_eq!(tool_calls.as_ref().map(Vec::len), Some(1));
    let out = serde_json::to_string(&req)?;
    assert!(out.contains(r#""content":null"#));
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
    let OpenAiChatMessage::Tool {
        content,
        tool_call_id,
        ..
    } = &req.messages[2]
    else {
        return Err("expected tool message".into());
    };
    assert_eq!(content, "result");
    assert_eq!(tool_call_id, "1");
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
        req.messages[0],
        OpenAiChatMessage::User {
            content: OpenAiChatContent::Parts(_),
            ..
        }
    ));
    Ok(())
}

#[test]
fn content_text_string() -> Result<(), Box<dyn std::error::Error>> {
    let json = r#"{"model": "gpt-4", "messages": [{"role": "user", "content": "simple text"}]}"#;
    let req: OpenAiChatCompletionRequest = serde_json::from_str(json)?;
    assert!(matches!(
        &req.messages[0],
        OpenAiChatMessage::User {
            content: OpenAiChatContent::Text(text),
            ..
        } if text == "simple text"
    ));
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

#[test]
fn ai_sdk_openai_compatible_content_parts_roundtrip() -> Result<(), Box<dyn std::error::Error>> {
    let json = r#"{
        "model": "gpt-4",
        "messages": [{
            "role": "user",
            "content": [
                {"type": "text", "text": "hello"},
                {"type": "input_audio", "input_audio": {"data": "UklGRg==", "format": "wav"}},
                {"type": "file", "file": {"filename": "document.pdf", "file_data": "data:application/pdf;base64,JVBERi0x"}}
            ]
        }]
    }"#;
    let req: OpenAiChatCompletionRequest = serde_json::from_str(json)?;
    let OpenAiChatMessage::User {
        content: OpenAiChatContent::Parts(parts),
        ..
    } = &req.messages[0]
    else {
        return Err("expected parts".into());
    };
    assert!(matches!(parts[1], OpenAiContentPart::InputAudio { .. }));
    assert!(matches!(parts[2], OpenAiContentPart::File { .. }));
    let out = serde_json::to_string(&req)?;
    assert!(out.contains("input_audio"));
    assert!(out.contains("file_data"));
    Ok(())
}

#[test]
fn false_stream_is_omitted_like_ai_sdk() -> Result<(), Box<dyn std::error::Error>> {
    let json = r#"{"model": "gpt-4", "messages": [{"role": "user", "content": "hi"}]}"#;
    let req: OpenAiChatCompletionRequest = serde_json::from_str(json)?;
    let out = serde_json::to_string(&req)?;
    assert!(!out.contains("stream"));
    Ok(())
}
