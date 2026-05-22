use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct OpenAiFilePart {
    pub filename: String,
    pub file_data: String,
}
