use serde::{Deserialize, Serialize};

use crate::claude::model::content::{AnthropicAssistantContent, AnthropicUserContent};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "role")]
pub enum AnthropicMessage {
    #[serde(rename = "user")]
    User { content: AnthropicUserContent },
    #[serde(rename = "assistant")]
    Assistant { content: AnthropicAssistantContent },
}
