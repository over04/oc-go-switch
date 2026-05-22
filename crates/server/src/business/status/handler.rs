use axum::{extract::State, http::StatusCode, response::Json};

use crate::business::{
    status::{
        dto::{
            account::AccountStatus,
            active_key::{ActiveKeyActionResponse, SetActiveKeyRequest},
            dashboard::DashboardStatusResponse,
            key::KeyStatusEntry,
            schedule::WorkspaceScheduleResponse,
            workspace::WorkspaceStatus,
            workspace_queue_status::WorkspaceQueueStatus,
        },
        service::refresh_workspace_pool,
    },
    workspace::{
        handle::KeyPoolHandle,
        key::{KeyStatus, PoolKey},
        record::WorkspacePool,
        scheduler::KeyPool,
        status::WorkspacePoolStatus,
    },
};

pub async fn dashboard_status(
    State(handle): State<KeyPoolHandle>,
) -> Json<DashboardStatusResponse> {
    let pool = handle.read().await;

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
    let available_workspaces = pool
        .workspaces
        .values()
        .filter(|workspace| workspace.status == WorkspacePoolStatus::Available)
        .count();
    let exhausted_workspaces = pool
        .workspaces
        .values()
        .filter(|workspace| workspace.status == WorkspacePoolStatus::Exhausted)
        .count();
    let unsubscribed_workspaces = pool
        .workspaces
        .values()
        .filter(|workspace| workspace.status == WorkspacePoolStatus::Unsubscribed)
        .count();
    let current_id = pool.current_key_id.clone();
    let go_workspaces = pool
        .workspaces
        .values()
        .map(|workspace| workspace_status_entry(&pool, workspace, current_id.as_deref()))
        .collect();

    Json(DashboardStatusResponse {
        total_keys: total,
        available_keys: available,
        available_workspaces,
        exhausted_workspaces,
        unsubscribed_workspaces,
        last_refresh_at: pool.last_refresh_at.clone(),
        go_workspaces,
    })
}

pub async fn workspace_schedule(
    State(handle): State<KeyPoolHandle>,
) -> Json<WorkspaceScheduleResponse> {
    let pool = handle.read().await;
    let current_id = pool.current_key_id.clone();

    Json(WorkspaceScheduleResponse {
        current_key_id: current_id.clone(),
        last_refresh_at: pool.last_refresh_at.clone(),
        accounts: account_status_entries(&pool, current_id.as_deref()),
    })
}

pub async fn set_active_key(
    State(handle): State<KeyPoolHandle>,
    Json(req): Json<SetActiveKeyRequest>,
) -> Result<Json<ActiveKeyActionResponse>, StatusCode> {
    if !handle.set_active_key(req.key_id).await {
        return Err(StatusCode::NOT_FOUND);
    }

    Ok(Json(ActiveKeyActionResponse { status: "ok" }))
}

pub async fn clear_active_key(
    State(handle): State<KeyPoolHandle>,
) -> Json<ActiveKeyActionResponse> {
    handle.clear_active_key().await;
    Json(ActiveKeyActionResponse { status: "ok" })
}

fn account_status_entries(pool: &KeyPool, current_id: Option<&str>) -> Vec<AccountStatus> {
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

        accounts[idx]
            .workspaces
            .push(workspace_status_entry(pool, workspace, current_id));
    }

    accounts
}

fn workspace_status_entry(
    pool: &KeyPool,
    workspace: &WorkspacePool,
    current_id: Option<&str>,
) -> WorkspaceStatus {
    WorkspaceStatus {
        id: workspace.id.clone(),
        name: workspace.name.clone(),
        status: workspace_status(workspace),
        is_current: pool.current_workspace_id.as_deref() == Some(workspace.id.as_str()),
        queue_position: queue_position(pool, &workspace.id),
        plan: workspace.plan.map(|p| format!("{:?}", p)),
        go_usage: workspace.go_usage.clone(),
        keys: workspace
            .keys
            .iter()
            .map(|key| key_status_entry(key, current_id))
            .collect(),
    }
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

pub async fn force_refresh(
    State(handle): State<KeyPoolHandle>,
) -> Result<&'static str, StatusCode> {
    refresh_workspace_pool(&handle).await
}
