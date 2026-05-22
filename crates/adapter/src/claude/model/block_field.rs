use serde::{Deserialize, Serialize};

use crate::claude::model::{
    image_source::AnthropicImageSource,
    request_field::{AnthropicCacheControl, AnthropicCitations},
};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum AnthropicToolResultContent {
    Text(String),
    Blocks(Vec<AnthropicNestedContentBlock>),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum AnthropicNestedContentBlock {
    #[serde(rename = "text")]
    Text { text: String },
    #[serde(rename = "image")]
    Image { source: AnthropicImageSource },
    #[serde(rename = "document")]
    Document {
        source: AnthropicImageSource,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        title: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        context: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        citations: Option<AnthropicCitations>,
    },
    #[serde(rename = "tool_reference")]
    ToolReference { tool_name: String },
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum AnthropicToolCallCaller {
    #[serde(rename = "direct")]
    Direct,
    #[serde(rename = "code_execution_20250825")]
    CodeExecution20250825 { tool_id: String },
    #[serde(rename = "code_execution_20260120")]
    CodeExecution20260120 { tool_id: String },
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum AnthropicServerToolName {
    #[serde(rename = "web_fetch")]
    WebFetch,
    #[serde(rename = "web_search")]
    WebSearch,
    #[serde(rename = "code_execution")]
    CodeExecution,
    #[serde(rename = "bash_code_execution")]
    BashCodeExecution,
    #[serde(rename = "text_editor_code_execution")]
    TextEditorCodeExecution,
    #[serde(rename = "tool_search_tool_regex")]
    ToolSearchToolRegex,
    #[serde(rename = "tool_search_tool_bm25")]
    ToolSearchToolBm25,
    #[serde(rename = "advisor")]
    Advisor,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AnthropicWebSearchResult {
    pub url: String,
    pub title: Option<String>,
    pub page_age: Option<String>,
    pub encrypted_content: String,
    #[serde(rename = "type")]
    pub result_type: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum AnthropicToolSearchResultContent {
    #[serde(rename = "tool_search_tool_search_result")]
    SearchResult {
        tool_references: Vec<AnthropicToolReference>,
    },
    #[serde(rename = "tool_search_tool_result_error")]
    Error { error_code: String },
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AnthropicToolReference {
    #[serde(rename = "type")]
    pub reference_type: AnthropicToolReferenceType,
    pub tool_name: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AnthropicToolReferenceType {
    ToolReference,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum AnthropicCodeExecutionToolResultContent {
    #[serde(rename = "code_execution_result")]
    Result {
        stdout: String,
        stderr: String,
        return_code: i64,
        content: Vec<AnthropicCodeExecutionOutput>,
    },
    #[serde(rename = "encrypted_code_execution_result")]
    EncryptedResult {
        encrypted_stdout: String,
        stderr: String,
        return_code: i64,
        content: Vec<AnthropicCodeExecutionOutput>,
    },
    #[serde(rename = "code_execution_tool_result_error")]
    Error { error_code: String },
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AnthropicCodeExecutionOutput {
    #[serde(rename = "type")]
    pub output_type: AnthropicCodeExecutionOutputType,
    pub file_id: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AnthropicCodeExecutionOutputType {
    CodeExecutionOutput,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum AnthropicTextEditorCodeExecutionToolResultContent {
    #[serde(rename = "text_editor_code_execution_tool_result_error")]
    Error { error_code: String },
    #[serde(rename = "text_editor_code_execution_create_result")]
    CreateResult { is_file_update: bool },
    #[serde(rename = "text_editor_code_execution_view_result")]
    ViewResult {
        content: String,
        file_type: String,
        num_lines: Option<i64>,
        start_line: Option<i64>,
        total_lines: Option<i64>,
    },
    #[serde(rename = "text_editor_code_execution_str_replace_result")]
    StrReplaceResult {
        lines: Option<Vec<String>>,
        new_lines: Option<i64>,
        new_start: Option<i64>,
        old_lines: Option<i64>,
        old_start: Option<i64>,
    },
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum AnthropicBashCodeExecutionToolResultContent {
    #[serde(rename = "bash_code_execution_result")]
    Result {
        stdout: String,
        stderr: String,
        return_code: i64,
        content: Vec<AnthropicBashCodeExecutionOutput>,
    },
    #[serde(rename = "bash_code_execution_tool_result_error")]
    Error { error_code: String },
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AnthropicBashCodeExecutionOutput {
    #[serde(rename = "type")]
    pub output_type: AnthropicBashCodeExecutionOutputType,
    pub file_id: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AnthropicBashCodeExecutionOutputType {
    BashCodeExecutionOutput,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum AnthropicAdvisorToolResultContent {
    #[serde(rename = "advisor_result")]
    Result { text: String },
    #[serde(rename = "advisor_redacted_result")]
    RedactedResult { encrypted_content: String },
    #[serde(rename = "advisor_tool_result_error")]
    Error { error_code: String },
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum AnthropicWebFetchToolResultContent {
    #[serde(rename = "web_fetch_result")]
    Result {
        url: String,
        retrieved_at: Option<String>,
        content: AnthropicWebFetchDocument,
    },
    #[serde(rename = "web_fetch_tool_result_error")]
    Error { error_code: String },
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AnthropicWebFetchDocument {
    #[serde(rename = "type")]
    pub document_type: AnthropicWebFetchDocumentType,
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub citations: Option<AnthropicCitations>,
    pub source: AnthropicWebFetchDocumentSource,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AnthropicWebFetchDocumentType {
    Document,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum AnthropicWebFetchDocumentSource {
    #[serde(rename = "base64")]
    Base64 {
        #[serde(rename = "media_type")]
        media_type: AnthropicWebFetchPdfMediaType,
        data: String,
    },
    #[serde(rename = "text")]
    Text {
        #[serde(rename = "media_type")]
        media_type: AnthropicWebFetchTextMediaType,
        data: String,
    },
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum AnthropicWebFetchPdfMediaType {
    #[serde(rename = "application/pdf")]
    ApplicationPdf,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum AnthropicWebFetchTextMediaType {
    #[serde(rename = "text/plain")]
    TextPlain,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum AnthropicMcpToolResultContent {
    Text(String),
    Blocks(Vec<AnthropicMcpToolResultTextBlock>),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AnthropicMcpToolResultTextBlock {
    #[serde(rename = "type")]
    pub block_type: AnthropicMcpToolResultTextBlockType,
    pub text: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AnthropicMcpToolResultTextBlockType {
    Text,
}

pub type AnthropicBlockCacheControl = Option<AnthropicCacheControl>;
