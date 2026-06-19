use axum::{
    routing::{delete, get, post, put},
    Router,
};

use crate::business::{status::handler, workspace::handle::WorkspaceSchedulerHandle};

/// 无鉴权健康检查路由。
pub fn public_router() -> Router<WorkspaceSchedulerHandle> {
    Router::new().route("/health", get(handler::health))
}

/// 仪表盘状态路由。
pub fn dashboard_router() -> Router<WorkspaceSchedulerHandle> {
    Router::new().route("/status", get(handler::dashboard_status))
}

/// 工作区调度路由。
pub fn workspace_router() -> Router<WorkspaceSchedulerHandle> {
    Router::new()
        .route("/status", get(handler::workspace_schedule))
        .route("/refresh", post(handler::force_refresh))
        .route("/current", put(handler::set_current_workspace))
        .route("/current", delete(handler::clear_current_workspace))
}
