use axum::{extract::State, response::Json};
use serde::Serialize;

use crate::opencode::types::GoUsage;
use crate::pool::key::{KeyStatus, PoolKey};
use crate::pool::pool::{KeyPool, KeyPoolHandle, WorkspacePool, WorkspacePoolStatus};

#[derive(Debug, Serialize)]
pub struct PoolStatusResponse {
    pub total_keys: usize,
    pub available_keys: usize,
    pub current_key_id: Option<String>,
    pub last_refresh_at: Option<String>,
    pub accounts: Vec<AccountStatus>,
}

#[derive(Debug, Serialize)]
pub struct AccountStatus {
    pub name: String,
    pub label: String,
    pub workspaces: Vec<WorkspaceStatus>,
}

#[derive(Debug, Serialize)]
pub struct WorkspaceStatus {
    pub id: String,
    pub name: String,
    pub status: WorkspaceQueueStatus,
    pub is_current: bool,
    pub queue_position: Option<usize>,
    pub plan: Option<String>,
    pub go_usage: Option<GoUsage>,
    pub keys: Vec<KeyStatusEntry>,
}

#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkspaceQueueStatus {
    Available,
    Exhausted,
    Unsubscribed,
}

#[derive(Debug, Serialize)]
pub struct KeyStatusEntry {
    pub id: String,
    pub masked: String,
    pub status: KeyStatus,
}

pub async fn pool_status(State(handle): State<KeyPoolHandle>) -> Json<PoolStatusResponse> {
    let pool = handle.read().await;
    let current_id = pool.current_key_id.clone();

    let mut accounts: Vec<AccountStatus> = Vec::new();
    let mut seen_accounts: std::collections::HashMap<String, usize> =
        std::collections::HashMap::new();

    for workspace in pool.workspaces.values() {
        let idx = match seen_accounts.get(&workspace.account_name) {
            Some(&i) => i,
            None => {
                let i = accounts.len();
                accounts.push(AccountStatus {
                    name: workspace.account_name.clone(),
                    label: workspace.account_label.clone(),
                    workspaces: Vec::new(),
                });
                seen_accounts.insert(workspace.account_name.clone(), i);
                i
            }
        };

        let acct = &mut accounts[idx];
        let queue_position = queue_position(&pool, &workspace.id);
        acct.workspaces.push(WorkspaceStatus {
            id: workspace.id.clone(),
            name: workspace.name.clone(),
            status: workspace_status(workspace),
            is_current: pool.current_workspace_id.as_deref() == Some(workspace.id.as_str()),
            queue_position,
            plan: workspace.plan.map(|p| format!("{:?}", p)),
            go_usage: workspace.go_usage.clone(),
            keys: workspace
                .keys
                .iter()
                .map(|key| key_status_entry(key, current_id.as_deref()))
                .collect(),
        });
    }

    let total: usize = pool
        .workspaces
        .values()
        .map(|workspace| workspace.keys.len())
        .sum();
    let available: usize = pool
        .workspaces
        .values()
        .filter(|workspace| workspace.status == WorkspacePoolStatus::Available)
        .map(|workspace| workspace.keys.len())
        .sum();

    Json(PoolStatusResponse {
        total_keys: total,
        available_keys: available,
        current_key_id: current_id,
        last_refresh_at: pool.last_refresh_at.clone(),
        accounts,
    })
}

fn workspace_status(workspace: &WorkspacePool) -> WorkspaceQueueStatus {
    match workspace.status {
        WorkspacePoolStatus::Available => WorkspaceQueueStatus::Available,
        WorkspacePoolStatus::Exhausted => WorkspaceQueueStatus::Exhausted,
        WorkspacePoolStatus::Unsubscribed => WorkspaceQueueStatus::Unsubscribed,
    }
}

fn queue_position(pool: &KeyPool, workspace_id: &str) -> Option<usize> {
    pool.workspace_queue
        .iter()
        .position(|id| id == workspace_id)
        .map(|idx| idx + 1)
}

fn key_status_entry(key: &PoolKey, current_id: Option<&str>) -> KeyStatusEntry {
    KeyStatusEntry {
        id: key.id.clone(),
        masked: key.masked_key(),
        status: if current_id == Some(key.id.as_str()) {
            KeyStatus::Active
        } else {
            KeyStatus::Idle
        },
    }
}

pub async fn health() -> &'static str {
    "ok"
}
