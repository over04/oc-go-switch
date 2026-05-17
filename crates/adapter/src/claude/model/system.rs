use serde::{Deserialize, Serialize};

use crate::claude::model::system_text::AnthropicSystemTextBlock;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum AnthropicSystemContent {
    Text(String),
    Blocks(Vec<AnthropicSystemTextBlock>),
}
