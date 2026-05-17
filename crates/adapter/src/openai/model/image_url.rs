use serde::{Deserialize, Serialize};

use crate::openai::model::image_detail::OpenAiImageDetail;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct OpenAiImageUrl {
    pub url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub detail: Option<OpenAiImageDetail>,
}
