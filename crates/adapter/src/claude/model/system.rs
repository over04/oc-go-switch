use serde::{Deserialize, Serialize};

use crate::claude::model::system_text::AnthropicSystemTextBlock;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AnthropicSystemContent(pub Vec<AnthropicSystemTextBlock>);
