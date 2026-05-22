use crate::common::config::{filter_action::FilterAction, image_filter::ImageFilterConfig};
use adapter::{
    claude::model::{
        block::AnthropicContentBlock, content::AnthropicUserContent, message::AnthropicMessage,
    },
    openai::model::{
        content::OpenAiChatContent, message::OpenAiChatMessage, part::OpenAiContentPart,
    },
};

/// 对 OpenAI 消息列表应用图片过滤。返回是否做了修改。
pub fn filter_openai_messages(
    messages: &mut [OpenAiChatMessage],
    model: &str,
    config: &ImageFilterConfig,
) -> bool {
    let Some(rule) = config.models.iter().find(|m| m.model == model) else {
        return false;
    };

    messages
        .iter_mut()
        .filter_map(openai_user_content_mut)
        .any(|content| filter_openai_content(content, &rule.action))
}

/// 对 Anthropic 消息列表应用图片过滤。返回是否做了修改。
pub fn filter_claude_messages(
    messages: &mut [AnthropicMessage],
    model: &str,
    config: &ImageFilterConfig,
) -> bool {
    let Some(rule) = config.models.iter().find(|m| m.model == model) else {
        return false;
    };

    messages
        .iter_mut()
        .filter_map(claude_user_content_mut)
        .any(|content| filter_claude_content(content, &rule.action))
}

fn openai_user_content_mut(message: &mut OpenAiChatMessage) -> Option<&mut OpenAiChatContent> {
    match message {
        OpenAiChatMessage::User { content, .. } => Some(content),
        _ => None,
    }
}

fn filter_openai_content(content: &mut OpenAiChatContent, action: &FilterAction) -> bool {
    let OpenAiChatContent::Parts(parts) = content else {
        return false;
    };

    let mut changed = false;
    let next = parts
        .iter()
        .filter_map(|part| match part {
            OpenAiContentPart::ImageUrl { .. } => match action {
                FilterAction::PassThrough => Some(part.clone()),
                FilterAction::Remove => {
                    changed = true;
                    None
                }
                FilterAction::Replace { replacement } => {
                    changed = true;
                    Some(OpenAiContentPart::Text {
                        text: replacement.clone(),
                        extra: Default::default(),
                    })
                }
            },
            part => Some(part.clone()),
        })
        .collect();

    if changed {
        *content = OpenAiChatContent::Parts(next);
    }

    changed
}

fn claude_user_content_mut(message: &mut AnthropicMessage) -> Option<&mut AnthropicUserContent> {
    match message {
        AnthropicMessage::User { content } => Some(content),
        AnthropicMessage::Assistant { .. } => None,
    }
}

fn filter_claude_content(content: &mut AnthropicUserContent, action: &FilterAction) -> bool {
    let mut changed = false;
    let next = content
        .blocks
        .iter()
        .filter_map(|block| match block {
            AnthropicContentBlock::Image { cache_control, .. } => match action {
                FilterAction::PassThrough => Some(block.clone()),
                FilterAction::Remove => {
                    changed = true;
                    None
                }
                FilterAction::Replace { replacement } => {
                    changed = true;
                    Some(AnthropicContentBlock::Text {
                        text: replacement.clone(),
                        cache_control: cache_control.clone(),
                    })
                }
            },
            block => Some(block.clone()),
        })
        .collect();

    if changed {
        content.blocks = next;
    }

    changed
}
