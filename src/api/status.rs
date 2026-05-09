use axum::{extract::State, response::Json};
use serde::Serialize;

use crate::opencode::types::GoUsage;
use crate::pool::key::KeyStatus;
use crate::pool::pool::KeyPoolHandle;

#[derive(Debug, Serialize)]
pub struct PoolStatusResponse {
    pub total_keys: usize,
    pub available_keys: usize,
    pub depleted_keys: usize,
    pub current_key_id: Option<String>,
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
    pub subscribed: bool,
    pub plan: Option<String>,
    pub go_usage: Option<GoUsage>,
    pub keys: Vec<KeyStatusEntry>,
}

#[derive(Debug, Serialize)]
pub struct KeyStatusEntry {
    pub id: String,
    pub masked: String,
    pub status: KeyStatus,
}

pub async fn pool_status(State(handle): State<KeyPoolHandle>) -> Json<PoolStatusResponse> {
    let pool = handle.read().await;
    let current_id = pool
        .selector
        .current_id()
        .and_then(|id| pool.keys.iter().find(|k| &k.id == id).map(|k| k.id.clone()));

    let mut accounts: Vec<AccountStatus> = Vec::new();
    let mut seen_accounts: std::collections::HashMap<String, usize> =
        std::collections::HashMap::new();

    for key in &pool.keys {
        let idx = match seen_accounts.get(&key.account_name) {
            Some(&i) => i,
            None => {
                let i = accounts.len();
                accounts.push(AccountStatus {
                    name: key.account_name.clone(),
                    label: key.account_label.clone(),
                    workspaces: Vec::new(),
                });
                seen_accounts.insert(key.account_name.clone(), i);
                i
            }
        };

        let acct = &mut accounts[idx];

        let ws_idx = acct
            .workspaces
            .iter()
            .position(|w| w.id == key.workspace_id);
        let ws_idx = match ws_idx {
            Some(i) => i,
            None => {
                let i = acct.workspaces.len();
                acct.workspaces.push(WorkspaceStatus {
                    id: key.workspace_id.clone(),
                    name: key.workspace_name.clone(),
                    subscribed: key.subscribed,
                    plan: key.plan.map(|p| format!("{:?}", p)),
                    go_usage: key.go_usage.clone(),
                    keys: Vec::new(),
                });
                i
            }
        };

        acct.workspaces[ws_idx].keys.push(KeyStatusEntry {
            id: key.id.clone(),
            masked: key.masked_key(),
            status: if Some(&key.id) == current_id.as_ref() {
                KeyStatus::Active
            } else {
                key.status()
            },
        });
    }

    let total = pool.keys.len();
    let depleted = pool.keys.iter().filter(|k| k.depleted).count();

    Json(PoolStatusResponse {
        total_keys: total,
        available_keys: total - depleted,
        depleted_keys: depleted,
        current_key_id: current_id,
        accounts,
    })
}

pub async fn health() -> &'static str {
    "ok"
}
