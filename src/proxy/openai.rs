use axum::{
    body::Bytes,
    extract::State,
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    Json,
};
use reqwest::header;
use serde::Serialize;
use tracing::{error, info};

use crate::model::{Direction, LogEntry};
use crate::pool::pool::KeyPoolHandle;
use chrono::Utc;

#[derive(Serialize)]
struct ProxyError {
    error: ProxyErrorDetail,
}

#[derive(Serialize)]
struct ProxyErrorDetail {
    message: String,
    #[serde(rename = "type")]
    error_type: String,
}

/// Handle OpenAI-compatible POST /v1/chat/completions
pub async fn chat_completions(
    State(handle): State<KeyPoolHandle>,
    _headers: HeaderMap,
    body: Bytes,
) -> impl IntoResponse {
    let start = std::time::Instant::now();
    let model: Option<String> = serde_json::from_slice::<serde_json::Value>(&body)
        .ok()
        .and_then(|v| v.get("model")?.as_str().map(String::from));

    let max_retries = 10;

    for attempt in 0..max_retries {
        let key = match handle.select_key().await {
            Some(k) => k,
            None => {
                handle.record_log(LogEntry {
                    timestamp: Utc::now().to_rfc3339(),
                    direction: Direction::OpenAI,
                    model,
                    status_code: 429,
                    duration_ms: start.elapsed().as_millis() as u64,
                    key_masked: "-".into(),
                    success: false,
                    error_message: Some("All API keys exhausted".into()),
                }).await;
                return (
                    StatusCode::TOO_MANY_REQUESTS,
                    Json(ProxyError {
                        error: ProxyErrorDetail {
                            message: "All API keys exhausted".into(),
                            error_type: "all_keys_exhausted".into(),
                        },
                    }),
                ).into_response();
            }
        };

        let upstream_url = format!("{}/chat/completions", handle.config().read().await.upstream.base_url);
        let mut upstream_body: serde_json::Value =
            serde_json::from_slice(&body).unwrap_or_default();
        upstream_body["api_key"] = serde_json::Value::String(key.key_value.clone());
        let upstream_bytes = serde_json::to_vec(&upstream_body).unwrap_or_default();

        info!(
            "Forwarding request via key {} (workspace={}, attempt={})",
            key.masked_key(),
            key.workspace_name,
            attempt + 1,
        );

        let client = reqwest::Client::new();
        let resp = client
            .post(&upstream_url)
            .header(header::AUTHORIZATION, format!("Bearer {}", key.key_value))
            .header(header::CONTENT_TYPE, "application/json")
            .body(upstream_bytes)
            .send()
            .await;

        match resp {
            Ok(r) => {
                let status = r.status();
                let resp_body = r.text().await.unwrap_or_default();

                if status.is_success() {
                    info!("Request succeeded via key {}", key.masked_key());
                    handle.record_log(LogEntry {
                        timestamp: Utc::now().to_rfc3339(),
                        direction: Direction::OpenAI,
                        model,
                        status_code: status.as_u16(),
                        duration_ms: start.elapsed().as_millis() as u64,
                        key_masked: key.masked_key(),
                        success: true,
                        error_message: None,
                    }).await;
                    return (StatusCode::OK, resp_body).into_response();
                }

                if is_quota_exhausted(status, &resp_body) {
                    info!("Key {} exhausted (status={}), switching...", key.masked_key(), status);
                    handle.mark_current_depleted().await;
                    continue;
                }

                handle.record_log(LogEntry {
                    timestamp: Utc::now().to_rfc3339(),
                    direction: Direction::OpenAI,
                    model,
                    status_code: status.as_u16(),
                    duration_ms: start.elapsed().as_millis() as u64,
                    key_masked: key.masked_key(),
                    success: false,
                    error_message: Some(resp_body.clone()),
                }).await;
                return (
                    StatusCode::from_u16(status.as_u16()).unwrap_or(StatusCode::BAD_GATEWAY),
                    resp_body,
                ).into_response();
            }
            Err(e) => {
                error!("Network error with key {}: {e}", key.masked_key());
                handle.record_log(LogEntry {
                    timestamp: Utc::now().to_rfc3339(),
                    direction: Direction::OpenAI,
                    model,
                    status_code: 502,
                    duration_ms: start.elapsed().as_millis() as u64,
                    key_masked: key.masked_key(),
                    success: false,
                    error_message: Some(format!("{e}")),
                }).await;
                return (
                    StatusCode::BAD_GATEWAY,
                    Json(ProxyError {
                        error: ProxyErrorDetail {
                            message: format!("Upstream error: {e}"),
                            error_type: "proxy_error".into(),
                        },
                    }),
                ).into_response();
            }
        }
    }

    handle.record_log(LogEntry {
        timestamp: Utc::now().to_rfc3339(),
        direction: Direction::OpenAI,
        model,
        status_code: 429,
        duration_ms: start.elapsed().as_millis() as u64,
        key_masked: "-".into(),
        success: false,
        error_message: Some("All API keys exhausted after retries".into()),
    }).await;
    (
        StatusCode::TOO_MANY_REQUESTS,
        Json(ProxyError {
            error: ProxyErrorDetail {
                message: "All API keys exhausted after retries".into(),
                error_type: "all_keys_exhausted".into(),
            },
        }),
    ).into_response()
}

fn is_quota_exhausted(status: StatusCode, body: &str) -> bool {
    status == StatusCode::PAYMENT_REQUIRED
        || status == StatusCode::TOO_MANY_REQUESTS
        || body.contains("insufficient")
        || body.contains("quota")
        || body.contains("balance")
        || body.contains("exceeded")
        || body.contains("exhausted")
        || body.contains("rate_limit")
}
