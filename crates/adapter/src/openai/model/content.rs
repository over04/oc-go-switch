use serde::{Deserialize, Serialize};

use crate::openai::model::part::OpenAiContentPart;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum OpenAiChatContent {
    Text(String),
    Parts(Vec<OpenAiContentPart>),
}
