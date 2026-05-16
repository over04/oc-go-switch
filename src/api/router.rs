use axum::{
    body::Body,
    extract::State,
    http::{header, StatusCode},
    middleware,
    response::Response,
    routing::{delete, get, post, put},
    Router,
};
use rust_embed::RustEmbed;
use tower_http::cors::CorsLayer;

use crate::pool::pool::KeyPoolHandle;

use super::{accounts, logs, models, status};
use crate::proxy::{claude, openai};

#[derive(RustEmbed)]
#[folder = "frontend/dist/"]
struct FrontendAssets;

pub fn build_router(pool_handle: KeyPoolHandle) -> Router {
    let public = Router::new()
        .route("/go/v1/chat/completions", post(openai::chat_completions))
        .route("/go/v1/messages", post(claude::messages))
        .route("/go/v1/models", get(models::list_models_v1))
        .route("/health", get(status::health))
        .with_state(pool_handle.clone());

    let admin = Router::new()
        .route("/pool/status", get(status::pool_status))
        .route("/api/accounts", get(accounts::list_accounts))
        .route("/api/accounts", post(accounts::add_account))
        .route("/api/accounts/{name}", put(accounts::edit_account))
        .route("/api/accounts/{name}", delete(accounts::delete_account))
        .route("/api/pool/active-key", put(accounts::set_active_key))
        .route("/api/pool/active-key", delete(accounts::clear_active_key))
        .route("/api/config", get(accounts::get_config))
        .route("/api/config", put(accounts::update_config))
        .route("/api/models", get(models::list_models))
        .route("/api/models/openai", get(models::list_openai_models))
        .route("/api/models/claude", get(models::list_claude_models))
        .route("/api/logs", get(logs::list_logs))
        .route("/api/logs", delete(logs::clear_logs))
        .route("/api/pool/refresh", post(accounts::force_refresh))
        .route_layer(middleware::from_fn_with_state(
            pool_handle.clone(),
            auth_middleware,
        ))
        .with_state(pool_handle.clone());

    let frontend = Router::new()
        .fallback(get(serve_frontend))
        .with_state(pool_handle);

    Router::new()
        .merge(public)
        .merge(admin)
        .merge(frontend)
        .layer(CorsLayer::permissive())
}

async fn auth_middleware(
    State(handle): State<KeyPoolHandle>,
    request: axum::http::Request<axum::body::Body>,
    next: middleware::Next,
) -> Result<Response, StatusCode> {
    let token = {
        let cfg = handle.config();
        let config = cfg.read().await;
        config.api_token.clone()
    };

    match token {
        None => Err(StatusCode::NOT_FOUND),
        Some(expected) => {
            let provided = request
                .headers()
                .get(header::AUTHORIZATION)
                .and_then(|v| v.to_str().ok())
                .and_then(|v| v.strip_prefix("Bearer "))
                .unwrap_or("");
            if provided == expected {
                Ok(next.run(request).await)
            } else {
                Err(StatusCode::UNAUTHORIZED)
            }
        }
    }
}

fn mime_from_path(path: &str) -> &'static str {
    match path.rsplit('.').next() {
        Some("html") => "text/html; charset=utf-8",
        Some("css") => "text/css; charset=utf-8",
        Some("js") => "application/javascript; charset=utf-8",
        Some("mjs") => "application/javascript; charset=utf-8",
        Some("json") => "application/json",
        Some("png") => "image/png",
        Some("svg") => "image/svg+xml",
        Some("ico") => "image/x-icon",
        Some("woff2") => "font/woff2",
        _ => "application/octet-stream",
    }
}

async fn serve_frontend(uri: axum::http::Uri) -> Response {
    let path = uri.path();
    let asset_path = path.trim_start_matches('/');

    if asset_path.starts_with("api/") || asset_path.starts_with("pool/") || asset_path.starts_with("go/") {
        return Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::from("未找到"))
            .unwrap();
    }

    let asset_path = if asset_path.is_empty() { "index.html" } else { asset_path };

    match FrontendAssets::get(asset_path) {
        Some(file) => Response::builder()
            .status(StatusCode::OK)
            .header(header::CONTENT_TYPE, mime_from_path(asset_path))
            .body(Body::from(file.data))
            .unwrap(),
        None => match FrontendAssets::get("index.html") {
            Some(index) => Response::builder()
                .status(StatusCode::OK)
                .header(header::CONTENT_TYPE, "text/html; charset=utf-8")
                .body(Body::from(index.data))
                .unwrap(),
            None => Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(Body::from("未找到"))
                .unwrap(),
        },
    }
}
