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
    let openai_api = Router::new()
        .route("/go/v1/chat/completions", post(openai::chat_completions))
        .route("/go/v1/models", get(models::list_models_v1))
        .route_layer(middleware::from_fn_with_state(
            pool_handle.clone(),
            openai_auth_middleware,
        ))
        .with_state(pool_handle.clone());

    let claude_api = Router::new()
        .route("/go/v1/messages", post(claude::messages))
        .route_layer(middleware::from_fn_with_state(
            pool_handle.clone(),
            claude_auth_middleware,
        ))
        .with_state(pool_handle.clone());

    let public = Router::new()
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
            admin_auth_middleware,
        ))
        .with_state(pool_handle.clone());

    let frontend = Router::new()
        .fallback(get(serve_frontend))
        .with_state(pool_handle);

    Router::new()
        .merge(public)
        .merge(openai_api)
        .merge(claude_api)
        .merge(admin)
        .merge(frontend)
        .layer(CorsLayer::permissive())
}

async fn admin_auth_middleware(
    State(handle): State<KeyPoolHandle>,
    request: axum::http::Request<axum::body::Body>,
    next: middleware::Next,
) -> Result<Response, StatusCode> {
    let expected = handle.config_snapshot().await.api_token;
    authorize_admin(&request, &expected)?;
    Ok(next.run(request).await)
}

async fn openai_auth_middleware(
    State(handle): State<KeyPoolHandle>,
    request: axum::http::Request<axum::body::Body>,
    next: middleware::Next,
) -> Result<Response, StatusCode> {
    let expected = handle.config_snapshot().await.api_token;
    authorize_openai(&request, &expected)?;
    Ok(next.run(request).await)
}

async fn claude_auth_middleware(
    State(handle): State<KeyPoolHandle>,
    request: axum::http::Request<axum::body::Body>,
    next: middleware::Next,
) -> Result<Response, StatusCode> {
    let expected = handle.config_snapshot().await.api_token;
    authorize_claude(&request, &expected)?;
    Ok(next.run(request).await)
}

fn authorize_admin(
    request: &axum::http::Request<axum::body::Body>,
    expected: &str,
) -> Result<(), StatusCode> {
    authorize_bearer(request, expected)
}

fn authorize_openai(
    request: &axum::http::Request<axum::body::Body>,
    expected: &str,
) -> Result<(), StatusCode> {
    authorize_bearer(request, expected)
}

fn authorize_claude(
    request: &axum::http::Request<axum::body::Body>,
    expected: &str,
) -> Result<(), StatusCode> {
    if x_api_key(request) == Some(expected) || bearer_token(request) == Some(expected) {
        return Ok(());
    }
    Err(StatusCode::UNAUTHORIZED)
}

fn authorize_bearer(
    request: &axum::http::Request<axum::body::Body>,
    expected: &str,
) -> Result<(), StatusCode> {
    if bearer_token(request) == Some(expected) {
        return Ok(());
    }
    Err(StatusCode::UNAUTHORIZED)
}

fn bearer_token(request: &axum::http::Request<axum::body::Body>) -> Option<&str> {
    request
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
}

fn x_api_key(request: &axum::http::Request<axum::body::Body>) -> Option<&str> {
    request
        .headers()
        .get("x-api-key")
        .and_then(|v| v.to_str().ok())
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

    if asset_path.starts_with("api/")
        || asset_path.starts_with("pool/")
        || asset_path.starts_with("go/")
    {
        return Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::from("未找到"))
            .unwrap();
    }

    let asset_path = if asset_path.is_empty() {
        "index.html"
    } else {
        asset_path
    };

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
