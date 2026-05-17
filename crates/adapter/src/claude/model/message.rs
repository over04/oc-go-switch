use serde::{Deserialize, Serialize};

use crate::claude::model::{content::AnthropicContent, role::AnthropicRole};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AnthropicMessage {
    pub role: AnthropicRole,
    pub content: AnthropicContent,
}
