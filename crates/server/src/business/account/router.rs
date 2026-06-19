use axum::{
    routing::{delete, get, post, put},
    Router,
};

use crate::business::{account::handler, workspace::handle::WorkspaceSchedulerHandle};

/// 账户管理路由。
///
/// 本层只写 `/api` 下的局部路径，完整前缀由 `init::router` 挂载。
pub fn router() -> Router<WorkspaceSchedulerHandle> {
    Router::new()
        .route("/accounts", get(handler::list_accounts))
        .route("/accounts", post(handler::add_account))
        .route("/accounts/{name}", put(handler::edit_account))
        .route("/accounts/{name}", delete(handler::delete_account))
}
