use serde::{Deserialize, Serialize};

use crate::claude::model::block::AnthropicContentBlock;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum AnthropicContent {
    Text(String),
    Blocks(Vec<AnthropicContentBlock>),
}
