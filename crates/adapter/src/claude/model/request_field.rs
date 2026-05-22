use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AnthropicCacheControl {
    #[serde(rename = "type")]
    pub cache_type: AnthropicCacheControlType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ttl: Option<AnthropicCacheTtl>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AnthropicCacheControlType {
    Ephemeral,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum AnthropicCacheTtl {
    #[serde(rename = "5m")]
    FiveMinutes,
    #[serde(rename = "1h")]
    OneHour,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AnthropicCitations {
    pub enabled: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum AnthropicThinking {
    #[serde(rename = "enabled")]
    Enabled { budget_tokens: u64 },
    #[serde(rename = "adaptive")]
    Adaptive {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        display: Option<AnthropicThinkingDisplay>,
    },
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AnthropicThinkingDisplay {
    Omitted,
    Summarized,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AnthropicOutputConfig {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effort: Option<AnthropicEffort>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub task_budget: Option<AnthropicTaskBudget>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub format: Option<AnthropicOutputFormat>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AnthropicEffort {
    Low,
    Medium,
    High,
    Xhigh,
    Max,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AnthropicTaskBudget {
    #[serde(rename = "type")]
    pub budget_type: AnthropicTaskBudgetType,
    pub total: u64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remaining: Option<u64>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AnthropicTaskBudgetType {
    Tokens,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AnthropicOutputFormat {
    #[serde(rename = "type")]
    pub format_type: AnthropicOutputFormatType,
    pub schema: Value,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AnthropicOutputFormatType {
    JsonSchema,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AnthropicSpeed {
    Fast,
    Standard,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AnthropicInferenceGeo {
    Us,
    Global,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AnthropicMetadata {
    pub user_id: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AnthropicMcpServer {
    #[serde(rename = "type")]
    pub server_type: AnthropicMcpServerType,
    pub name: String,
    pub url: String,
    #[serde(
        default,
        deserialize_with = "deserialize_nullish",
        skip_serializing_if = "Option::is_none"
    )]
    pub authorization_token: Option<Option<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tool_configuration: Option<AnthropicMcpToolConfiguration>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AnthropicMcpServerType {
    Url,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AnthropicMcpToolConfiguration {
    #[serde(
        default,
        deserialize_with = "deserialize_nullish",
        skip_serializing_if = "Option::is_none"
    )]
    pub allowed_tools: Option<Option<Vec<String>>>,
    #[serde(
        default,
        deserialize_with = "deserialize_nullish",
        skip_serializing_if = "Option::is_none"
    )]
    pub enabled: Option<Option<bool>>,
}

fn deserialize_nullish<'de, D, T>(deserializer: D) -> Result<Option<Option<T>>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    Option::<T>::deserialize(deserializer).map(Some)
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum AnthropicContainer {
    Id(String),
    Skills {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        id: Option<String>,
        skills: Vec<AnthropicContainerSkill>,
    },
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AnthropicContainerSkill {
    #[serde(rename = "type")]
    pub skill_type: AnthropicContainerSkillType,
    pub skill_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AnthropicContainerSkillType {
    Anthropic,
    Custom,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AnthropicContextManagement {
    pub edits: Vec<AnthropicContextManagementEdit>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum AnthropicContextManagementEdit {
    #[serde(rename = "clear_tool_uses_20250919")]
    ClearToolUses {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        trigger: Option<AnthropicContextManagementTrigger>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        keep: Option<AnthropicToolUsesTrigger>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        clear_at_least: Option<AnthropicInputTokensTrigger>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        clear_tool_inputs: Option<bool>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        exclude_tools: Option<Vec<String>>,
    },
    #[serde(rename = "clear_thinking_20251015")]
    ClearThinking {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        keep: Option<AnthropicThinkingKeep>,
    },
    #[serde(rename = "compact_20260112")]
    Compact {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        trigger: Option<AnthropicInputTokensTrigger>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pause_after_compaction: Option<bool>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        instructions: Option<String>,
    },
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum AnthropicContextManagementTrigger {
    #[serde(rename = "input_tokens")]
    InputTokens { value: u64 },
    #[serde(rename = "tool_uses")]
    ToolUses { value: u64 },
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AnthropicToolUsesTrigger {
    #[serde(rename = "type")]
    pub trigger_type: AnthropicToolUsesTriggerType,
    pub value: u64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AnthropicToolUsesTriggerType {
    ToolUses,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AnthropicInputTokensTrigger {
    #[serde(rename = "type")]
    pub trigger_type: AnthropicInputTokensTriggerType,
    pub value: u64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AnthropicInputTokensTriggerType {
    InputTokens,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum AnthropicThinkingKeep {
    All(AnthropicAllThinkingKeep),
    Turns(AnthropicThinkingTurnsKeep),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum AnthropicAllThinkingKeep {
    #[serde(rename = "all")]
    All,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AnthropicThinkingTurnsKeep {
    #[serde(rename = "type")]
    pub keep_type: AnthropicThinkingTurnsKeepType,
    pub value: u64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AnthropicThinkingTurnsKeepType {
    ThinkingTurns,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum AnthropicToolChoice {
    #[serde(rename = "auto")]
    Auto {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        disable_parallel_tool_use: Option<bool>,
    },
    #[serde(rename = "any")]
    Any {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        disable_parallel_tool_use: Option<bool>,
    },
    #[serde(rename = "tool")]
    Tool {
        name: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        disable_parallel_tool_use: Option<bool>,
    },
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum AnthropicTool {
    Custom(AnthropicCustomTool),
    Provider(AnthropicProviderTool),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AnthropicCustomTool {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub input_schema: Value,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cache_control: Option<AnthropicCacheControl>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub eager_input_streaming: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub strict: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub defer_loading: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allowed_callers: Option<Vec<AnthropicAllowedCaller>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum AnthropicAllowedCaller {
    #[serde(rename = "direct")]
    Direct,
    #[serde(rename = "code_execution_20250825")]
    CodeExecution20250825,
    #[serde(rename = "code_execution_20260120")]
    CodeExecution20260120,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum AnthropicProviderTool {
    #[serde(rename = "code_execution_20250522")]
    CodeExecution20250522 {
        name: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        cache_control: Option<AnthropicCacheControl>,
    },
    #[serde(rename = "code_execution_20250825")]
    CodeExecution20250825 { name: String },
    #[serde(rename = "code_execution_20260120")]
    CodeExecution20260120 { name: String },
    #[serde(rename = "computer_20250124")]
    Computer20250124 {
        name: String,
        display_width_px: u64,
        display_height_px: u64,
        display_number: u64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        cache_control: Option<AnthropicCacheControl>,
    },
    #[serde(rename = "computer_20241022")]
    Computer20241022 {
        name: String,
        display_width_px: u64,
        display_height_px: u64,
        display_number: u64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        cache_control: Option<AnthropicCacheControl>,
    },
    #[serde(rename = "computer_20251124")]
    Computer20251124 {
        name: String,
        display_width_px: u64,
        display_height_px: u64,
        display_number: u64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enable_zoom: Option<bool>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        cache_control: Option<AnthropicCacheControl>,
    },
    #[serde(rename = "text_editor_20250124")]
    TextEditor20250124 {
        name: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        cache_control: Option<AnthropicCacheControl>,
    },
    #[serde(rename = "text_editor_20241022")]
    TextEditor20241022 {
        name: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        cache_control: Option<AnthropicCacheControl>,
    },
    #[serde(rename = "text_editor_20250429")]
    TextEditor20250429 {
        name: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        cache_control: Option<AnthropicCacheControl>,
    },
    #[serde(rename = "text_editor_20250728")]
    TextEditor20250728 {
        name: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        max_characters: Option<u64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        cache_control: Option<AnthropicCacheControl>,
    },
    #[serde(rename = "bash_20250124")]
    Bash20250124 {
        name: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        cache_control: Option<AnthropicCacheControl>,
    },
    #[serde(rename = "bash_20241022")]
    Bash20241022 {
        name: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        cache_control: Option<AnthropicCacheControl>,
    },
    #[serde(rename = "memory_20250818")]
    Memory20250818 { name: String },
    #[serde(rename = "web_fetch_20250910")]
    WebFetch20250910 {
        name: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        max_uses: Option<u64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        allowed_domains: Option<Vec<String>>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        blocked_domains: Option<Vec<String>>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        citations: Option<AnthropicCitations>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        max_content_tokens: Option<u64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        cache_control: Option<AnthropicCacheControl>,
    },
    #[serde(rename = "web_fetch_20260209")]
    WebFetch20260209 {
        name: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        max_uses: Option<u64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        allowed_domains: Option<Vec<String>>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        blocked_domains: Option<Vec<String>>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        citations: Option<AnthropicCitations>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        max_content_tokens: Option<u64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        cache_control: Option<AnthropicCacheControl>,
    },
    #[serde(rename = "web_search_20250305")]
    WebSearch20250305 {
        name: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        max_uses: Option<u64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        allowed_domains: Option<Vec<String>>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        blocked_domains: Option<Vec<String>>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        user_location: Option<AnthropicUserLocation>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        cache_control: Option<AnthropicCacheControl>,
    },
    #[serde(rename = "web_search_20260209")]
    WebSearch20260209 {
        name: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        max_uses: Option<u64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        allowed_domains: Option<Vec<String>>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        blocked_domains: Option<Vec<String>>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        user_location: Option<AnthropicUserLocation>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        cache_control: Option<AnthropicCacheControl>,
    },
    #[serde(rename = "tool_search_tool_regex_20251119")]
    ToolSearchRegex20251119 { name: String },
    #[serde(rename = "tool_search_tool_bm25_20251119")]
    ToolSearchBm2520251119 { name: String },
    #[serde(rename = "advisor_20260301")]
    Advisor20260301 {
        name: AnthropicAdvisorName,
        model: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        max_uses: Option<u64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        caching: Option<AnthropicAdvisorCaching>,
    },
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AnthropicUserLocation {
    #[serde(rename = "type")]
    pub location_type: AnthropicUserLocationType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AnthropicUserLocationType {
    Approximate,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum AnthropicAdvisorName {
    #[serde(rename = "advisor")]
    Advisor,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AnthropicAdvisorCaching {
    #[serde(rename = "type")]
    pub caching_type: AnthropicCacheControlType,
    pub ttl: AnthropicCacheTtl,
}
