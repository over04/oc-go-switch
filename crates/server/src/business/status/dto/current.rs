use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct SetCurrentWorkspaceRequest {
    pub workspace_id: String,
}

#[derive(Debug, Serialize)]
pub struct CurrentWorkspaceActionResponse {
    pub status: &'static str,
}
