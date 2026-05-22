use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum AnthropicTextSourceMediaType {
    #[serde(rename = "text/plain")]
    TextPlain,
}
