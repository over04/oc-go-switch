use axum::{
    body::Body,
    http::{header, StatusCode},
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
    let api_routes = Router::new()
        .route("/v1/chat/completions", post(openai::chat_completions))
        .route("/v1/messages", post(claude::messages))
        .route("/pool/status", get(status::pool_status))
        .route("/health", get(status::health))
        // Account management
        .route("/api/accounts", get(accounts::list_accounts))
        .route("/api/accounts", post(accounts::add_account))
        .route("/api/accounts/{name}", put(accounts::edit_account))
        .route("/api/accounts/{name}", delete(accounts::delete_account))
        // Active key management
        .route("/api/pool/active-key", put(accounts::set_active_key))
        .route("/api/pool/active-key", delete(accounts::clear_active_key))
        // Config management
        .route("/api/config", get(accounts::get_config))
        .route("/api/config", put(accounts::update_config))
        // Model list
        .route("/api/models", get(models::list_models))
        .route("/api/models/openai", get(models::list_openai_models))
        .route("/api/models/claude", get(models::list_claude_models))
        // Logs
        .route("/api/logs", get(logs::list_logs))
        .route("/api/logs", delete(logs::clear_logs))
        // Force refresh
        .route("/api/pool/refresh", post(accounts::force_refresh));

    let frontend_routes = Router::new().fallback(get(serve_frontend));

    Router::new()
        .merge(frontend_routes)
        .merge(api_routes)
        .layer(CorsLayer::permissive())
        .with_state(pool_handle)
}

fn mime_from_path(path: &str) -> &'static str {
    if path.ends_with(".html") {
        "text/html; charset=utf-8"
    } else if path.ends_with(".js") || path.ends_with(".mjs") {
        "application/javascript"
    } else if path.ends_with(".css") {
        "text/css"
    } else if path.ends_with(".svg") {
        "image/svg+xml"
    } else if path.ends_with(".png") {
        "image/png"
    } else if path.ends_with(".woff2") {
        "font/woff2"
    } else if path.ends_with(".woff") {
        "font/woff"
    } else if path.ends_with(".json") {
        "application/json"
    } else {
        "application/octet-stream"
    }
}

async fn serve_frontend(uri: axum::http::Uri) -> Response {
    let path = uri.path().trim_start_matches('/');
    let asset_path = if path.is_empty() { "index.html" } else { path };

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
                .body(Body::from("Not found"))
                .unwrap(),
        },
    }
}
