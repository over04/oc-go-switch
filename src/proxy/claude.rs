use axum::{
    body::Bytes,
    extract::State,
    http::{header, StatusCode},
    response::Response,
};
use chrono::Utc;
use tracing::{error, info};

use crate::model::{Direction, LogEntry};
use crate::pool::pool::KeyPoolHandle;
use super::error;
use super::stream;

pub async fn messages(
    State(handle): State<KeyPoolHandle>,
    body: Bytes,
) -> Response {
    let start = std::time::Instant::now();

    // Validate request before touching the pool
    let (model, is_stream) = match validate_claude_request(&body) {
        Ok(v) => v,
        Err(resp) => return resp,
    };

    let max_retries = handle.config().read().await.max_retries;

    for _attempt in 0..=max_retries {
        let key = match handle.select_key().await {
            Some(k) => k,
            None => {
                handle.record_log(LogEntry {
                    timestamp: Utc::now().to_rfc3339(),
                    direction: Direction::Claude,
                    model: Some(model.clone()),
                    status_code: 429,
                    duration_ms: start.elapsed().as_millis() as u64,
                    key_masked: "-".into(),
                    success: false,
                    error_message: Some("All API keys exhausted".into()),
                    stream: is_stream,
                }).await;
                return error::anthropic_error(
                    StatusCode::TOO_MANY_REQUESTS,
                    "All API keys exhausted",
                    "server_error",
                );
            }
        };

        let upstream_url = format!("{}/messages", handle.config().read().await.go.base_url);

        info!(
            "Forwarding Claude request via key {} (workspace={}, attempt={})",
            key.masked_key(),
            key.workspace_name,
            _attempt + 1,
        );

        let resp = handle
            .http_client
            .post(&upstream_url)
            .header("x-api-key", &key.key_value)
            .header("anthropic-version", "2023-06-01")
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .body(body.clone())
            .send()
            .await;

        match resp {
            Ok(r) => {
                let status = r.status();

                if status.is_success() {
                    info!("Claude request succeeded via key {}", key.masked_key());
                    handle.record_log(LogEntry {
                        timestamp: Utc::now().to_rfc3339(),
                        direction: Direction::Claude,
                        model: Some(model),
                        status_code: status.as_u16(),
                        duration_ms: start.elapsed().as_millis() as u64,
                        key_masked: key.masked_key(),
                        success: true,
                        error_message: None,
                        stream: is_stream,
                    }).await;

                    if is_stream {
                        return stream::forward_sse_stream(r, false);
                    }
                    return stream::forward_json_response(r).await;
                }

                // Non-success — read body to check for quota exhaustion
                let resp_body = r.text().await.unwrap_or_default();

                if is_claude_quota_exhausted(status, &resp_body) {
                    info!(
                        "Key {} exhausted on Claude endpoint (status={}), switching...",
                        key.masked_key(),
                        status
                    );
                    handle.mark_current_depleted().await;
                    continue;
                }

                // Non-quota upstream error — forward with proper Content-Type
                handle.record_log(LogEntry {
                    timestamp: Utc::now().to_rfc3339(),
                    direction: Direction::Claude,
                    model: Some(model),
                    status_code: status.as_u16(),
                    duration_ms: start.elapsed().as_millis() as u64,
                    key_masked: key.masked_key(),
                    success: false,
                    error_message: Some(resp_body.clone()),
                    stream: is_stream,
                }).await;
                return json_response(
                    StatusCode::from_u16(status.as_u16())
                        .unwrap_or(StatusCode::BAD_GATEWAY),
                    &resp_body,
                );
            }
            Err(e) => {
                error!("Network error with key {}: {e}", key.masked_key());
                handle.record_log(LogEntry {
                    timestamp: Utc::now().to_rfc3339(),
                    direction: Direction::Claude,
                    model: Some(model),
                    status_code: 502,
                    duration_ms: start.elapsed().as_millis() as u64,
                    key_masked: key.masked_key(),
                    success: false,
                    error_message: Some(format!("{e}")),
                    stream: is_stream,
                }).await;
                return error::anthropic_error(
                    StatusCode::BAD_GATEWAY,
                    format!("Upstream error: {e}"),
                    "server_error",
                );
            }
        }
    }

    // All retries exhausted
    handle.record_log(LogEntry {
        timestamp: Utc::now().to_rfc3339(),
        direction: Direction::Claude,
        model: Some(model),
        status_code: 429,
        duration_ms: start.elapsed().as_millis() as u64,
        key_masked: "-".into(),
        success: false,
        error_message: Some("All API keys exhausted after retries".into()),
        stream: is_stream,
    }).await;
    error::anthropic_error(
        StatusCode::TOO_MANY_REQUESTS,
        "All API keys exhausted after retries",
        "server_error",
    )
}

/// Validate that the request body has required Anthropic fields.
/// Returns (model, is_stream) or an error response.
#[allow(clippy::result_large_err)]
fn validate_claude_request(body: &Bytes) -> Result<(String, bool), Response> {
    let v: serde_json::Value = serde_json::from_slice(body).unwrap_or_default();

    let model = v
        .get("model")
        .and_then(|m| m.as_str())
        .filter(|s| !s.is_empty())
        .map(String::from)
        .ok_or_else(|| {
            error::anthropic_error(
                StatusCode::BAD_REQUEST,
                "'model' is required",
                "invalid_request_error",
            )
        })?;

    let messages = v.get("messages").and_then(|m| m.as_array());
    match messages {
        Some(arr) if !arr.is_empty() => {}
        _ => {
            return Err(error::anthropic_error(
                StatusCode::BAD_REQUEST,
                "'messages' is required and must be a non-empty array",
                "invalid_request_error",
            ));
        }
    }

    let max_tokens = v.get("max_tokens").and_then(|m| m.as_u64()).unwrap_or(0);
    if max_tokens == 0 {
        return Err(error::anthropic_error(
            StatusCode::BAD_REQUEST,
            "'max_tokens' is required",
            "invalid_request_error",
        ));
    }

    let stream = v.get("stream").and_then(|s| s.as_bool()).unwrap_or(false);

    Ok((model, stream))
}

fn is_claude_quota_exhausted(status: StatusCode, body: &str) -> bool {
    status == StatusCode::PAYMENT_REQUIRED
        || status == StatusCode::TOO_MANY_REQUESTS
        || body.contains("insufficient")
        || body.contains("quota")
        || body.contains("balance")
        || body.contains("exceeded")
        || body.contains("exhausted")
        || body.contains("overloaded_error")
}

/// Return a text body as JSON with proper Content-Type header.
fn json_response(status: StatusCode, body: &str) -> Response {
    Response::builder()
        .status(status)
        .header(header::CONTENT_TYPE, "application/json")
        .body(axum::body::Body::from(body.to_owned()))
        .unwrap_or_else(|_| {
            Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(axum::body::Body::from("Internal proxy error"))
                .unwrap_or_else(|_| Response::new(axum::body::Body::empty()))
        })
}
