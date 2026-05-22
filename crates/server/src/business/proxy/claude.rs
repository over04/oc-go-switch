use axum::{body::Bytes, extract::State, http::StatusCode, response::Response};
use chrono::Utc;
use tracing::{info, warn};

use crate::{
    business::{
        proxy::{error, filter, quota, response, stream},
        workspace::handle::KeyPoolHandle,
    },
    common::model::{direction::Direction, log::LogEntry},
};
use adapter::claude::model::messages_request::AnthropicMessagesRequest;

pub async fn messages(
    State(handle): State<KeyPoolHandle>,
    headers: axum::http::HeaderMap,
    body: Bytes,
) -> Response {
    let start = std::time::Instant::now();

    let mut req: AnthropicMessagesRequest = match serde_json::from_slice(&body) {
        Ok(r) => r,
        Err(e) => {
            return error::anthropic_error(
                StatusCode::BAD_REQUEST,
                format!("请求体无效: {e}"),
                "invalid_request_error",
            );
        }
    };

    if let Err(msg) = req.validate() {
        return error::anthropic_error(StatusCode::BAD_REQUEST, msg, "invalid_request_error");
    }

    let model = req.model.clone();
    let is_stream = req.stream;
    let config = handle.runtime_config();
    let clients = handle.clients();
    let image_filter_config = config.image_filter.clone();
    let max_retries = config.max_retries;
    let base_url = config.go.base_url.clone();

    if !image_filter_config.models.is_empty()
        && filter::filter_claude_messages(&mut req.messages, &model, &image_filter_config)
    {
        info!("图片过滤已应用于模型 '{model}'");
    }

    let upstream_bytes = match serde_json::to_vec(&req) {
        Ok(b) => b,
        Err(e) => {
            tracing::error!("序列化上游请求失败: {e}");
            return error::anthropic_error(
                StatusCode::BAD_REQUEST,
                "无效的请求体",
                "invalid_request_error",
            );
        }
    };
    for _attempt in 0..=max_retries {
        let key = match handle.select_key_or_refresh().await {
            Some(k) => k,
            None => {
                handle
                    .record_log(LogEntry {
                        timestamp: Utc::now().to_rfc3339(),
                        direction: Direction::Claude,
                        model: Some(model.clone()),
                        status_code: 429,
                        duration_ms: start.elapsed().as_millis() as u64,
                        key_masked: "-".into(),
                        success: false,
                        error_message: Some("没有可用 Go 工作区".into()),
                        stream: is_stream,
                    })
                    .await;
                return error::anthropic_error(
                    StatusCode::TOO_MANY_REQUESTS,
                    "没有可用 Go 工作区",
                    "server_error",
                );
            }
        };

        let upstream_url = format!("{base_url}/messages");

        info!(
            "转发 Claude 请求 key={} workspace={} 第{}次尝试",
            key.masked_key(),
            key.workspace_name,
            _attempt + 1,
        );

        let request = clients
            .proxy
            .post(&upstream_url)
            .header(reqwest::header::ACCEPT_ENCODING, "identity")
            .header("x-api-key", &key.key_value)
            .header("anthropic-version", "2023-06-01")
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .body(upstream_bytes.clone());

        let resp = apply_anthropic_beta(request, &headers).send().await;

        match resp {
            Ok(r) => {
                let status = r.status();

                if status.is_success() {
                    info!("Claude 请求成功 key={}", key.masked_key());
                    handle
                        .record_log(LogEntry {
                            timestamp: Utc::now().to_rfc3339(),
                            direction: Direction::Claude,
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
                        return stream::sse_response_from_upstream(r);
                    }
                    return stream::json_response_from_upstream(r).await;
                }

                let resp_body = r.text().await.unwrap_or_default();

                if quota::is_quota_exhausted(status, &resp_body) {
                    info!(
                        "key 已耗尽 key={} status={} 切换中...",
                        key.masked_key(),
                        status
                    );
                    handle.drop_workspace(&key.workspace_id).await;
                    continue;
                }

                handle
                    .record_log(LogEntry {
                        timestamp: Utc::now().to_rfc3339(),
                        direction: Direction::Claude,
                        model: Some(model),
                        status_code: status.as_u16(),
                        duration_ms: start.elapsed().as_millis() as u64,
                        key_masked: key.masked_key(),
                        success: false,
                        error_message: Some(resp_body.clone()),
                        stream: is_stream,
                    })
                    .await;
                return response::json_response(
                    StatusCode::from_u16(status.as_u16()).unwrap_or(StatusCode::BAD_GATEWAY),
                    &resp_body,
                );
            }
            Err(e) => {
                warn!("网络错误 key={}: {e}，切换 key...", key.masked_key());
                handle.drop_workspace(&key.workspace_id).await;
                handle
                    .record_log(LogEntry {
                        timestamp: Utc::now().to_rfc3339(),
                        direction: Direction::Claude,
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
            direction: Direction::Claude,
            model: Some(model),
            status_code: 429,
            duration_ms: start.elapsed().as_millis() as u64,
            key_masked: "-".into(),
            success: false,
            error_message: Some("重试耗尽，所有 API key 不可用".into()),
            stream: is_stream,
        })
        .await;
    error::anthropic_error(
        StatusCode::TOO_MANY_REQUESTS,
        "重试耗尽，所有 API key 不可用",
        "server_error",
    )
}

fn apply_anthropic_beta(
    request: reqwest::RequestBuilder,
    headers: &axum::http::HeaderMap,
) -> reqwest::RequestBuilder {
    match headers.get("anthropic-beta") {
        Some(value) => request.header("anthropic-beta", value.clone()),
        None => request,
    }
}
