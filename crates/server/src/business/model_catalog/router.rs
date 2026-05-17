use axum::{routing::get, Router};

use crate::business::{model_catalog::handler, workspace::handle::KeyPoolHandle};

pub fn openai_router() -> Router<KeyPoolHandle> {
    Router::new().route("/go/v1/models", get(handler::list_models_v1))
}

pub fn admin_router() -> Router<KeyPoolHandle> {
    Router::new()
        .route("/models", get(handler::list_models))
        .route("/models/openai", get(handler::list_openai_models))
        .route("/models/claude", get(handler::list_claude_models))
}
