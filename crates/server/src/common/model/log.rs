use serde::Serialize;

use crate::common::model::direction::Direction;

#[derive(Debug, Clone, Serialize)]
pub struct LogEntry {
    pub timestamp: String,
    pub direction: Direction,
    pub model: Option<String>,
    pub status_code: u16,
    pub duration_ms: u64,
    pub key_masked: String,
    pub success: bool,
    pub error_message: Option<String>,
    pub stream: bool,
}
