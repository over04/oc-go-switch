use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::claude::model::{
    block_field::{
        AnthropicAdvisorToolResultContent, AnthropicBashCodeExecutionToolResultContent,
        AnthropicBlockCacheControl, AnthropicCodeExecutionToolResultContent,
        AnthropicMcpToolResultContent, AnthropicServerToolName,
        AnthropicTextEditorCodeExecutionToolResultContent, AnthropicToolCallCaller,
        AnthropicToolResultContent, AnthropicToolSearchResultContent,
        AnthropicWebFetchToolResultContent, AnthropicWebSearchResult,
    },
    image_source::AnthropicImageSource,
    request_field::AnthropicCitations,
};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum AnthropicContentBlock {
    #[serde(rename = "text")]
    Text {
        text: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        cache_control: AnthropicBlockCacheControl,
    },
    #[serde(rename = "image")]
    Image {
        source: AnthropicImageSource,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        cache_control: AnthropicBlockCacheControl,
    },
    #[serde(rename = "document")]
    Document {
        source: AnthropicImageSource,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        title: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        context: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        citations: Option<AnthropicCitations>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        cache_control: AnthropicBlockCacheControl,
    },
    #[serde(rename = "tool_result")]
    ToolResult {
        tool_use_id: String,
        content: AnthropicToolResultContent,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        is_error: Option<bool>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        cache_control: AnthropicBlockCacheControl,
    },
    #[serde(rename = "tool_use")]
    ToolUse {
        id: String,
        name: String,
        input: Value,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        caller: Option<AnthropicToolCallCaller>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        cache_control: AnthropicBlockCacheControl,
    },
    #[serde(rename = "thinking")]
    Thinking { thinking: String, signature: String },
    #[serde(rename = "redacted_thinking")]
    RedactedThinking { data: String },
    #[serde(rename = "server_tool_use")]
    ServerToolUse {
        id: String,
        name: AnthropicServerToolName,
        input: Value,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        cache_control: AnthropicBlockCacheControl,
    },
    #[serde(rename = "code_execution_tool_result")]
    CodeExecutionToolResult {
        tool_use_id: String,
        content: AnthropicCodeExecutionToolResultContent,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        cache_control: AnthropicBlockCacheControl,
    },
    #[serde(rename = "text_editor_code_execution_tool_result")]
    TextEditorCodeExecutionToolResult {
        tool_use_id: String,
        content: AnthropicTextEditorCodeExecutionToolResultContent,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        cache_control: AnthropicBlockCacheControl,
    },
    #[serde(rename = "bash_code_execution_tool_result")]
    BashCodeExecutionToolResult {
        tool_use_id: String,
        content: AnthropicBashCodeExecutionToolResultContent,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        cache_control: AnthropicBlockCacheControl,
    },
    #[serde(rename = "web_search_tool_result")]
    WebSearchToolResult {
        tool_use_id: String,
        content: Vec<AnthropicWebSearchResult>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        cache_control: AnthropicBlockCacheControl,
    },
    #[serde(rename = "web_fetch_tool_result")]
    WebFetchToolResult {
        tool_use_id: String,
        content: AnthropicWebFetchToolResultContent,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        cache_control: AnthropicBlockCacheControl,
    },
    #[serde(rename = "tool_search_tool_result")]
    ToolSearchToolResult {
        tool_use_id: String,
        content: AnthropicToolSearchResultContent,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        cache_control: AnthropicBlockCacheControl,
    },
    #[serde(rename = "advisor_tool_result")]
    AdvisorToolResult {
        tool_use_id: String,
        content: AnthropicAdvisorToolResultContent,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        cache_control: AnthropicBlockCacheControl,
    },
    #[serde(rename = "mcp_tool_use")]
    McpToolUse {
        id: String,
        name: String,
        server_name: String,
        input: Value,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        cache_control: AnthropicBlockCacheControl,
    },
    #[serde(rename = "mcp_tool_result")]
    McpToolResult {
        tool_use_id: String,
        is_error: bool,
        content: AnthropicMcpToolResultContent,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        cache_control: AnthropicBlockCacheControl,
    },
    #[serde(rename = "compaction")]
    Compaction {
        content: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        cache_control: AnthropicBlockCacheControl,
    },
}

impl AnthropicContentBlock {
    pub fn is_user_message_block(&self) -> bool {
        matches!(
            self,
            Self::Text { .. }
                | Self::Image { .. }
                | Self::Document { .. }
                | Self::ToolResult { .. }
        )
    }

    pub fn is_assistant_message_block(&self) -> bool {
        matches!(
            self,
            Self::Text { .. }
                | Self::Thinking { .. }
                | Self::RedactedThinking { .. }
                | Self::ToolUse { .. }
                | Self::ServerToolUse { .. }
                | Self::CodeExecutionToolResult { .. }
                | Self::TextEditorCodeExecutionToolResult { .. }
                | Self::BashCodeExecutionToolResult { .. }
                | Self::WebSearchToolResult { .. }
                | Self::WebFetchToolResult { .. }
                | Self::ToolSearchToolResult { .. }
                | Self::AdvisorToolResult { .. }
                | Self::McpToolUse { .. }
                | Self::McpToolResult { .. }
                | Self::Compaction { .. }
        )
    }
}
