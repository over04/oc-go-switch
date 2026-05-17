use axum::{middleware, routing::get, Router};
use tower_http::cors::CorsLayer;

use crate::{
    business::{
        account, configuration, log, model_catalog, proxy::router as proxy_router, status,
        workspace::handle::KeyPoolHandle,
    },
    common::{auth::middleware::admin_auth_middleware, frontend::assets::serve_frontend},
};

/// 应用路由聚合层。
///
/// 功能域 router 只声明局部路径；这里统一挂载 `/api`、协议入口和前端 fallback。
pub fn build_router(pool_handle: KeyPoolHandle) -> Router {
    let api_router = Router::new()
        .merge(account::router::router())
        .merge(configuration::router::router())
        .merge(model_catalog::router::admin_router())
        .merge(log::router::router())
        .nest("/dashboard", status::router::dashboard_router())
        .nest("/workspaces", status::router::workspace_router())
        .route_layer(middleware::from_fn_with_state(
            pool_handle.clone(),
            admin_auth_middleware,
        ))
        .with_state(pool_handle.clone());

    Router::new()
        .merge(status::router::public_router().with_state(pool_handle.clone()))
        .merge(proxy_router::openai_router(pool_handle.clone()))
        .merge(proxy_router::claude_router(pool_handle.clone()))
        .nest("/api", api_router)
        .merge(
            Router::new()
                .fallback(get(serve_frontend))
                .with_state(pool_handle),
        )
        .layer(CorsLayer::permissive())
}
