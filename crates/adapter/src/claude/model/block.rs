use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::claude::model::image_source::AnthropicImageSource;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum AnthropicContentBlock {
    #[serde(rename = "text")]
    Text {
        text: String,
        #[serde(flatten)]
        extra: HashMap<String, Value>,
    },
    #[serde(rename = "image")]
    Image {
        source: AnthropicImageSource,
        #[serde(flatten)]
        extra: HashMap<String, Value>,
    },
    #[serde(untagged)]
    Other(Value),
}
