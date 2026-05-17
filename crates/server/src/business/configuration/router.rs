use axum::{
    routing::{get, put},
    Router,
};

use crate::business::{configuration::handler, workspace::handle::KeyPoolHandle};

pub fn router() -> Router<KeyPoolHandle> {
    Router::new()
        .route("/config", get(handler::get_config))
        .route("/config", put(handler::update_config))
}
