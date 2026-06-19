use axum::{extract::State, http::StatusCode, response::Json};

use crate::business::{
    status::{
        dto::{
            account::AccountStatus,
            current::{CurrentWorkspaceActionResponse, SetCurrentWorkspaceRequest},
            dashboard::DashboardStatusResponse,
            schedule::WorkspaceScheduleResponse,
            workspace::WorkspaceStatus,
            workspace_queue_status::WorkspaceQueueStatus,
        },
        service::refresh_workspace_pool,
    },
    workspace::{
        handle::WorkspaceSchedulerHandle, record::WorkspacePool, scheduler::WorkspaceScheduler,
        status::WorkspacePoolStatus,
    },
};

pub async fn dashboard_status(
    State(handle): State<WorkspaceSchedulerHandle>,
) -> Json<DashboardStatusResponse> {
    let pool = handle.read().await;

    let total_workspaces = pool.workspaces.len();
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
    let mut go_workspaces: Vec<_> = pool
        .workspaces
        .values()
        .map(|workspace| workspace_status_entry(&pool, workspace))
        .collect();
    go_workspaces.sort_by(|left, right| left.id.cmp(&right.id));

    Json(DashboardStatusResponse {
        total_workspaces,
        available_workspaces,
        exhausted_workspaces,
        unsubscribed_workspaces,
        last_refresh_at: pool.last_refresh_at.clone(),
        go_workspaces,
    })
}

pub async fn workspace_schedule(
    State(handle): State<WorkspaceSchedulerHandle>,
) -> Json<WorkspaceScheduleResponse> {
    let pool = handle.read().await;

    Json(WorkspaceScheduleResponse {
        current_workspace_id: pool.current_workspace_id.clone(),
        last_refresh_at: pool.last_refresh_at.clone(),
        accounts: account_status_entries(&pool),
    })
}

pub async fn set_current_workspace(
    State(handle): State<WorkspaceSchedulerHandle>,
    Json(req): Json<SetCurrentWorkspaceRequest>,
) -> Result<Json<CurrentWorkspaceActionResponse>, StatusCode> {
    if !handle.set_current_workspace(req.workspace_id).await {
        return Err(StatusCode::NOT_FOUND);
    }

    Ok(Json(CurrentWorkspaceActionResponse { status: "ok" }))
}

pub async fn clear_current_workspace(
    State(handle): State<WorkspaceSchedulerHandle>,
) -> Json<CurrentWorkspaceActionResponse> {
    handle.clear_current_workspace().await;
    Json(CurrentWorkspaceActionResponse { status: "ok" })
}

fn account_status_entries(pool: &WorkspaceScheduler) -> Vec<AccountStatus> {
    let mut accounts: Vec<AccountStatus> = Vec::new();
    let mut seen_accounts: std::collections::HashMap<String, usize> =
        std::collections::HashMap::new();

    let mut workspaces: Vec<&WorkspacePool> = pool.workspaces.values().collect();
    workspaces.sort_by(|left, right| {
        left.account_name
            .cmp(&right.account_name)
            .then_with(|| left.id.cmp(&right.id))
    });

    for workspace in workspaces {
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
            .push(workspace_status_entry(pool, workspace));
    }

    accounts
}

fn workspace_status_entry(pool: &WorkspaceScheduler, workspace: &WorkspacePool) -> WorkspaceStatus {
    WorkspaceStatus {
        id: workspace.id.clone(),
        name: workspace.name.clone(),
        status: workspace_status(workspace),
        is_current: pool.current_workspace_id.as_deref() == Some(workspace.id.as_str()),
        queue_position: queue_position(pool, &workspace.id),
        plan: workspace.plan.map(|p| format!("{:?}", p)),
        go_usage: workspace.go_usage.clone(),
        credential_masked: workspace.credential.masked(),
    }
}

fn workspace_status(workspace: &WorkspacePool) -> WorkspaceQueueStatus {
    match workspace.status {
        WorkspacePoolStatus::Available => WorkspaceQueueStatus::Available,
        WorkspacePoolStatus::Exhausted => WorkspaceQueueStatus::Exhausted,
        WorkspacePoolStatus::Unsubscribed => WorkspaceQueueStatus::Unsubscribed,
    }
}

fn queue_position(pool: &WorkspaceScheduler, workspace_id: &str) -> Option<usize> {
    pool.workspace_queue
        .iter()
        .position(|id| id == workspace_id)
        .map(|idx| idx + 1)
}

pub async fn health() -> &'static str {
    "ok"
}

pub async fn force_refresh(
    State(handle): State<WorkspaceSchedulerHandle>,
) -> Result<&'static str, StatusCode> {
    refresh_workspace_pool(&handle).await
}
