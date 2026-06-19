use axum::{
    extract::State,
    http::{header, Request, StatusCode},
    middleware,
    response::Response,
};

use crate::business::workspace::handle::WorkspaceSchedulerHandle;

pub async fn admin_auth_middleware(
    State(handle): State<WorkspaceSchedulerHandle>,
    request: Request<axum::body::Body>,
    next: middleware::Next,
) -> Result<Response, StatusCode> {
    let expected = handle.runtime_config();
    authorize_admin(&request, &expected.api_token)?;
    Ok(next.run(request).await)
}

pub async fn openai_auth_middleware(
    State(handle): State<WorkspaceSchedulerHandle>,
    request: Request<axum::body::Body>,
    next: middleware::Next,
) -> Result<Response, StatusCode> {
    let expected = handle.runtime_config();
    authorize_openai(&request, &expected.api_token)?;
    Ok(next.run(request).await)
}

pub async fn claude_auth_middleware(
    State(handle): State<WorkspaceSchedulerHandle>,
    request: Request<axum::body::Body>,
    next: middleware::Next,
) -> Result<Response, StatusCode> {
    let expected = handle.runtime_config();
    authorize_claude(&request, &expected.api_token)?;
    Ok(next.run(request).await)
}

fn authorize_admin(request: &Request<axum::body::Body>, expected: &str) -> Result<(), StatusCode> {
    authorize_bearer(request, expected)
}

fn authorize_openai(request: &Request<axum::body::Body>, expected: &str) -> Result<(), StatusCode> {
    authorize_bearer(request, expected)
}

fn authorize_claude(request: &Request<axum::body::Body>, expected: &str) -> Result<(), StatusCode> {
    if x_api_key(request) == Some(expected) || bearer_token(request) == Some(expected) {
        return Ok(());
    }
    Err(StatusCode::UNAUTHORIZED)
}

fn authorize_bearer(request: &Request<axum::body::Body>, expected: &str) -> Result<(), StatusCode> {
    if bearer_token(request) == Some(expected) {
        return Ok(());
    }
    Err(StatusCode::UNAUTHORIZED)
}

fn bearer_token(request: &Request<axum::body::Body>) -> Option<&str> {
    request
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.strip_prefix("Bearer "))
}

fn x_api_key(request: &Request<axum::body::Body>) -> Option<&str> {
    request
        .headers()
        .get("x-api-key")
        .and_then(|value| value.to_str().ok())
}
