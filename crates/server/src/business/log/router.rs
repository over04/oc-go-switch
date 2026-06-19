use axum::{
    routing::{delete, get},
    Router,
};

use crate::business::{log::handler, workspace::handle::WorkspaceSchedulerHandle};

pub fn router() -> Router<WorkspaceSchedulerHandle> {
    Router::new()
        .route("/logs", get(handler::list_logs))
        .route("/logs", delete(handler::clear_logs))
}
