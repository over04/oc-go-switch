use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct OpenAiInputAudio {
    pub data: String,
    pub format: OpenAiInputAudioFormat,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum OpenAiInputAudioFormat {
    Wav,
    Mp3,
}
