use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::openai::model::{
    file::OpenAiFilePart, image_url::OpenAiImageUrl, input_audio::OpenAiInputAudio,
};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum OpenAiContentPart {
    #[serde(rename = "text")]
    Text {
        text: String,
        #[serde(flatten)]
        extra: HashMap<String, Value>,
    },
    #[serde(rename = "image_url")]
    ImageUrl {
        image_url: OpenAiImageUrl,
        #[serde(flatten)]
        extra: HashMap<String, Value>,
    },
    #[serde(rename = "input_audio")]
    InputAudio {
        input_audio: OpenAiInputAudio,
        #[serde(flatten)]
        extra: HashMap<String, Value>,
    },
    #[serde(rename = "file")]
    File {
        file: OpenAiFilePart,
        #[serde(flatten)]
        extra: HashMap<String, Value>,
    },
}
