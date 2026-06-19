use axum::{middleware, routing::post, Router};

use crate::{
    business::{
        model_catalog,
        proxy::{claude, openai},
        workspace::handle::WorkspaceSchedulerHandle,
    },
    common::auth::middleware::{claude_auth_middleware, openai_auth_middleware},
};

/// OpenAI 协议入口路由。
///
/// `/go/v1` 是对外协议入口前缀，保持原路径；本层只挂协议内路由和 OpenAI 鉴权。
pub fn openai_router(pool_handle: WorkspaceSchedulerHandle) -> Router {
    Router::new()
        .merge(model_catalog::router::openai_router())
        .route("/go/v1/chat/completions", post(openai::chat_completions))
        .route_layer(middleware::from_fn_with_state(
            pool_handle.clone(),
            openai_auth_middleware,
        ))
        .with_state(pool_handle)
}

/// Anthropic 协议入口路由。
///
/// 鉴权函数独立于 OpenAI，但读取同一个 `api_token` 配置值。
pub fn claude_router(pool_handle: WorkspaceSchedulerHandle) -> Router {
    Router::new()
        .route("/go/v1/messages", post(claude::messages))
        .route_layer(middleware::from_fn_with_state(
            pool_handle.clone(),
            claude_auth_middleware,
        ))
        .with_state(pool_handle)
}
