use std::collections::HashMap;

use crate::common::config::{filter_action::FilterAction, image_filter::ImageFilterConfig};
use adapter::{
    claude::model::{
        block::AnthropicContentBlock, content::AnthropicContent, message::AnthropicMessage,
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

    let mut modified = false;
    for msg in messages.iter_mut() {
        if let Some(OpenAiChatContent::Parts(ref parts)) = msg.content {
            let mut new_parts: Vec<OpenAiContentPart> = Vec::new();

            for part in parts {
                match part {
                    OpenAiContentPart::ImageUrl { .. } => match rule.action {
                        FilterAction::PassThrough => {
                            new_parts.push(part.clone());
                        }
                        FilterAction::Remove => {
                            modified = true;
                        }
                        FilterAction::Replace => {
                            modified = true;
                            let text = rule
                                .replacement
                                .clone()
                                .unwrap_or_else(|| "[图片已移除]".to_string());
                            new_parts.push(OpenAiContentPart::Text {
                                text,
                                extra: HashMap::new(),
                            });
                        }
                    },
                    other => {
                        new_parts.push(other.clone());
                    }
                }
            }

            if modified {
                if new_parts.is_empty() {
                    msg.content = Some(OpenAiChatContent::Text("[所有内容已移除]".to_string()));
                } else if new_parts.len() == 1 {
                    if let OpenAiContentPart::Text { text, .. } = &new_parts[0] {
                        msg.content = Some(OpenAiChatContent::Text(text.clone()));
                    } else {
                        msg.content = Some(OpenAiChatContent::Parts(new_parts));
                    }
                } else {
                    msg.content = Some(OpenAiChatContent::Parts(new_parts));
                }
            }
        }
    }

    modified
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

    let mut modified = false;
    for msg in messages.iter_mut() {
        if let AnthropicContent::Blocks(ref blocks) = msg.content {
            let mut new_blocks: Vec<AnthropicContentBlock> = Vec::new();

            for block in blocks {
                match block {
                    AnthropicContentBlock::Image { .. } => match rule.action {
                        FilterAction::PassThrough => {
                            new_blocks.push(block.clone());
                        }
                        FilterAction::Remove => {
                            modified = true;
                        }
                        FilterAction::Replace => {
                            modified = true;
                            let text = rule
                                .replacement
                                .clone()
                                .unwrap_or_else(|| "[图片已移除]".to_string());
                            new_blocks.push(AnthropicContentBlock::Text {
                                text,
                                extra: HashMap::new(),
                            });
                        }
                    },
                    other => {
                        new_blocks.push(other.clone());
                    }
                }
            }

            if modified {
                if new_blocks.is_empty() {
                    msg.content = AnthropicContent::Text("[所有内容已移除]".to_string());
                } else if new_blocks.len() == 1 {
                    if let AnthropicContentBlock::Text { text, .. } = &new_blocks[0] {
                        msg.content = AnthropicContent::Text(text.clone());
                    } else {
                        msg.content = AnthropicContent::Blocks(new_blocks);
                    }
                } else {
                    msg.content = AnthropicContent::Blocks(new_blocks);
                }
            }
        }
    }

    modified
}
