use serde::Serialize;

use crate::business::status::dto::workspace::WorkspaceStatus;

#[derive(Debug, Serialize)]
pub struct AccountStatus {
    pub name: String,
    pub label: String,
    pub workspaces: Vec<WorkspaceStatus>,
}
