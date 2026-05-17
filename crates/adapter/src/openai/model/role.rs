use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum OpenAiChatRole {
    System,
    User,
    Assistant,
    Tool,
    Function,
    Developer,
    #[serde(other)]
    Unknown,
}
