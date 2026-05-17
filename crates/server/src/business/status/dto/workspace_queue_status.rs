use serde::Serialize;

#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkspaceQueueStatus {
    Available,
    Exhausted,
    Unsubscribed,
}
