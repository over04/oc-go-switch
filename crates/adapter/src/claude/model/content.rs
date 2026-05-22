use serde::{Deserialize, Deserializer, Serialize};

use crate::claude::model::block::AnthropicContentBlock;

#[derive(Debug, Clone, Serialize)]
#[serde(transparent)]
pub struct AnthropicUserContent {
    pub blocks: Vec<AnthropicContentBlock>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(transparent)]
pub struct AnthropicAssistantContent {
    pub blocks: Vec<AnthropicContentBlock>,
}

impl<'de> Deserialize<'de> for AnthropicUserContent {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let blocks = Vec::<AnthropicContentBlock>::deserialize(deserializer)?;
        if blocks
            .iter()
            .all(AnthropicContentBlock::is_user_message_block)
        {
            Ok(Self { blocks })
        } else {
            Err(serde::de::Error::custom(
                "invalid Anthropic user message content block",
            ))
        }
    }
}

impl<'de> Deserialize<'de> for AnthropicAssistantContent {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let blocks = Vec::<AnthropicContentBlock>::deserialize(deserializer)?;
        if blocks
            .iter()
            .all(AnthropicContentBlock::is_assistant_message_block)
        {
            Ok(Self { blocks })
        } else {
            Err(serde::de::Error::custom(
                "invalid Anthropic assistant message content block",
            ))
        }
    }
}
