use serde::{Deserialize, Serialize};

use crate::claude::model::text_source_media_type::AnthropicTextSourceMediaType;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum AnthropicImageSource {
    #[serde(rename = "base64")]
    Base64 { media_type: String, data: String },
    #[serde(rename = "url")]
    Url { url: String },
    #[serde(rename = "text")]
    Text {
        media_type: AnthropicTextSourceMediaType,
        data: String,
    },
}
