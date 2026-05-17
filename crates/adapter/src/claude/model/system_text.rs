use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::claude::model::system_block_kind::AnthropicSystemBlockType;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AnthropicSystemTextBlock {
    #[serde(rename = "type")]
    pub block_type: AnthropicSystemBlockType,
    pub text: String,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}
