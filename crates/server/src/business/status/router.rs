use axum::{
    routing::{delete, get, post, put},
    Router,
};

use crate::business::{status::handler, workspace::handle::KeyPoolHandle};

/// 无鉴权健康检查路由。
pub fn public_router() -> Router<KeyPoolHandle> {
    Router::new().route("/health", get(handler::health))
}

/// 仪表盘状态路由。
pub fn dashboard_router() -> Router<KeyPoolHandle> {
    Router::new().route("/status", get(handler::dashboard_status))
}

/// 工作区调度路由。
pub fn workspace_router() -> Router<KeyPoolHandle> {
    Router::new()
        .route("/status", get(handler::workspace_schedule))
        .route("/refresh", post(handler::force_refresh))
        .route("/active-key", put(handler::set_active_key))
        .route("/active-key", delete(handler::clear_active_key))
}
