use axum::{
    routing::{delete, get},
    Router,
};

use crate::business::{log::handler, workspace::handle::KeyPoolHandle};

pub fn router() -> Router<KeyPoolHandle> {
    Router::new()
        .route("/logs", get(handler::list_logs))
        .route("/logs", delete(handler::clear_logs))
}
