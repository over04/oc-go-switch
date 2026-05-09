use axum::{
    body::Bytes,
    extract::State,
    http::{header, StatusCode},
    response::Response,
};
use chrono::Utc;
use std::collections::HashSet;
use tracing::{info, warn};

use crate::config::ImageFilterConfig;
use crate::model::{Direction, LogEntry};
use crate::pool::pool::KeyPoolHandle;
use crate::protocol::openai::ChatCompletionRequest;

use super::error;
use super::filter;
use super::stream;

pub async fn chat_completions(
    State(handle): State<KeyPoolHandle>,
    _headers: axum::http::HeaderMap,
    body: Bytes,
) -> Response {
    let start = std::time::Instant::now();

    let mut req: ChatCompletionRequest = match serde_json::from_slice(&body) {
        Ok(r) => r,
        Err(e) => {
            return error::openai_error(
                StatusCode::BAD_REQUEST,
                format!("Invalid request body: {e}"),
                "invalid_request_error",
                None,
                None,
            );
        }
    };

    if let Err(msg) = req.validate() {
        return error::openai_error(
            StatusCode::BAD_REQUEST,
            msg,
            "invalid_request_error",
            Some(if msg.contains("model") { "model" } else { "messages" }),
            Some("missing_required_parameter"),
        );
    }

    let model = req.model.clone();
    let is_stream = req.stream;

    {
        let cfg = handle.config();
        let config = cfg.read().await;
        let image_filter_config: &ImageFilterConfig = &config.image_filter;
        if !image_filter_config.models.is_empty()
            && filter::filter_openai_messages(&mut req.messages, &model, image_filter_config)
        {
            info!("Image filter applied to model '{model}'");
        }
    }

    let max_retries = handle.config().read().await.max_retries;
    let mut depleted_ids: HashSet<String> = HashSet::new();

    for _attempt in 0..=max_retries {
        let key = match handle.select_key(Some(&depleted_ids)).await {
            Some(k) => k,
            None => {
                handle
                    .record_log(LogEntry {
                        timestamp: Utc::now().to_rfc3339(),
                        direction: Direction::OpenAI,
                        model: Some(model.clone()),
                        status_code: 429,
                        duration_ms: start.elapsed().as_millis() as u64,
                        key_masked: "-".into(),
                        success: false,
                        error_message: Some("All API keys exhausted".into()),
                        stream: is_stream,
                    })
                    .await;
                return error::openai_error(
                    StatusCode::TOO_MANY_REQUESTS,
                    "All API keys exhausted",
                    "server_error",
                    None,
                    None,
                );
            }
        };

        let upstream_url =
            format!("{}/chat/completions", handle.config().read().await.go.base_url);

        let mut upstream_value = serde_json::to_value(&req).unwrap_or_default();
        upstream_value["api_key"] =
            serde_json::Value::String(key.key_value.clone());
        let upstream_bytes = serde_json::to_vec(&upstream_value).unwrap_or_default();

        info!(
            "Forwarding request via key {} (workspace={}, attempt={})",
            key.masked_key(),
            key.workspace_name,
            _attempt + 1,
        );

        let resp = handle
            .http_client
            .post(&upstream_url)
            .header(
                reqwest::header::AUTHORIZATION,
                format!("Bearer {}", key.key_value),
            )
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .body(upstream_bytes)
            .send()
            .await;

        match resp {
            Ok(r) => {
                let status = r.status();

                if status.is_success() {
                    info!("Request succeeded via key {}", key.masked_key());
                    handle
                        .record_log(LogEntry {
                            timestamp: Utc::now().to_rfc3339(),
                            direction: Direction::OpenAI,
                            model: Some(model),
                            status_code: status.as_u16(),
                            duration_ms: start.elapsed().as_millis() as u64,
                            key_masked: key.masked_key(),
                            success: true,
                            error_message: None,
                            stream: is_stream,
                        })
                        .await;

                    if is_stream {
                        return stream::forward_sse_stream(r, true);
                    }
                    return stream::forward_json_response(r).await;
                }

                let resp_body = r.text().await.unwrap_or_default();

                if is_quota_exhausted(status, &resp_body) {
                    info!(
                        "Key {} exhausted (status={}), switching...",
                        key.masked_key(),
                        status
                    );
                    depleted_ids.insert(key.id.clone());
                    handle.mark_current_depleted().await;
                    continue;
                }

                handle
                    .record_log(LogEntry {
                        timestamp: Utc::now().to_rfc3339(),
                        direction: Direction::OpenAI,
                        model: Some(model),
                        status_code: status.as_u16(),
                        duration_ms: start.elapsed().as_millis() as u64,
                        key_masked: key.masked_key(),
                        success: false,
                        error_message: Some(resp_body.clone()),
                        stream: is_stream,
                    })
                    .await;
                return json_response(
                    StatusCode::from_u16(status.as_u16())
                        .unwrap_or(StatusCode::BAD_GATEWAY),
                    &resp_body,
                );
            }
            Err(e) => {
                warn!(
                    "Network error with key {}: {e}, retrying...",
                    key.masked_key()
                );
                handle
                    .record_log(LogEntry {
                        timestamp: Utc::now().to_rfc3339(),
                        direction: Direction::OpenAI,
                        model: Some(model.clone()),
                        status_code: 502,
                        duration_ms: start.elapsed().as_millis() as u64,
                        key_masked: key.masked_key(),
                        success: false,
                        error_message: Some(format!("{e}")),
                        stream: is_stream,
                    })
                    .await;
                continue;
            }
        }
    }

    handle
        .record_log(LogEntry {
            timestamp: Utc::now().to_rfc3339(),
            direction: Direction::OpenAI,
            model: Some(model),
            status_code: 429,
            duration_ms: start.elapsed().as_millis() as u64,
            key_masked: "-".into(),
            success: false,
            error_message: Some("All API keys exhausted after retries".into()),
            stream: is_stream,
        })
        .await;
    error::openai_error(
        StatusCode::TOO_MANY_REQUESTS,
        "All API keys exhausted after retries",
        "server_error",
        None,
        None,
    )
}

/// Only real balance/insufficient errors. Generic rate_limit, overloaded,
/// or context_length_exceeded (400) are NOT quota signals.
fn is_quota_exhausted(status: StatusCode, body: &str) -> bool {
    (status == StatusCode::PAYMENT_REQUIRED || status == StatusCode::TOO_MANY_REQUESTS)
        && (body.contains("insufficient")
            || body.contains("quota")
            || body.contains("balance"))
}

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
