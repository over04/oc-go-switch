use serde::{Deserialize, Serialize};

use crate::claude::model::{
    request_field::AnthropicCacheControl, system_block_kind::AnthropicSystemBlockType,
};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AnthropicSystemTextBlock {
    #[serde(rename = "type")]
    pub block_type: AnthropicSystemBlockType,
    pub text: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cache_control: Option<AnthropicCacheControl>,
}
