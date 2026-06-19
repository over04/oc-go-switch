use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct SetAffinityWorkspaceRequest {
    pub workspace_id: String,
}

#[derive(Debug, Serialize)]
pub struct AffinityActionResponse {
    pub status: &'static str,
}
