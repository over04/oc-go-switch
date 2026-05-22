use std::{
    collections::{HashMap, VecDeque},
    sync::{Arc, Mutex},
};

use adapter::opencode::model::{go_usage::GoUsage, subscription_plan::SubscriptionPlan};
use axum::{
    body::{Body, Bytes},
    extract::State,
    http::{HeaderMap, Request, StatusCode},
    response::IntoResponse,
    routing::post,
    Json, Router,
};
use serde_json::{json, Value};
use tokio::net::TcpListener;
use tower::ServiceExt;

use crate::{
    business::{
        log::store::LogStore,
        proxy::router::{claude_router, openai_router},
        workspace::{
            handle::KeyPoolHandle, key::PoolKey, record::WorkspacePool, scheduler::KeyPool,
            status::WorkspacePoolStatus,
        },
    },
    common::config::{
        filter_action::FilterAction, fixed::FixedConfig, go::GoConfig,
        image_filter::ImageFilterConfig, image_filter_model::ImageFilterModel,
        runtime::RuntimeConfig, store::ConfigStore, Config,
    },
};

#[tokio::test]
async fn openai_proxy_matches_ai_sdk_shape_and_keeps_filtering(
) -> Result<(), Box<dyn std::error::Error>> {
    let upstream = FakeUpstream::start_openai().await?;
    let app = openai_router(handle(
        upstream.base_url(),
        ImageFilterConfig {
            models: vec![ImageFilterModel {
                model: "gpt-test".to_string(),
                action: FilterAction::Remove,
            }],
        },
    )?);

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/go/v1/chat/completions")
                .header("authorization", "Bearer admin-token")
                .header("anthropic-beta", "kept-for-provider")
                .header("content-type", "application/json")
                .body(Body::from(
                    json!({
                        "model": "gpt-test",
                        "messages": [{
                            "role": "user",
                            "content": [
                                {"type": "text", "text": "hello"},
                                {"type": "image_url", "image_url": {"url": "https://example.com/a.png"}}
                            ]
                        }],
                        "temperature": 0.2
                    })
                    .to_string(),
                ))?,
        )
        .await?;

    assert_eq!(response.status(), StatusCode::OK);
    let captured = upstream.take();
    assert_eq!(
        captured.authorization.as_deref(),
        Some("Bearer sk-upstream-key-0000")
    );
    assert!(!captured.body.as_object().unwrap().contains_key("api_key"));
    assert_eq!(captured.body["temperature"], json!(0.2));
    assert_eq!(
        captured.body["messages"][0]["content"],
        json!([{"type": "text", "text": "hello"}])
    );
    Ok(())
}

#[tokio::test]
async fn claude_proxy_matches_ai_sdk_shape_and_keeps_filtering(
) -> Result<(), Box<dyn std::error::Error>> {
    let upstream = FakeUpstream::start_claude().await?;
    let app = claude_router(handle(
        upstream.base_url(),
        ImageFilterConfig {
            models: vec![ImageFilterModel {
                model: "claude-test".to_string(),
                action: FilterAction::Replace {
                    replacement: "[image omitted]".to_string(),
                },
            }],
        },
    )?);

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/go/v1/messages")
                .header("x-api-key", "admin-token")
                .header("anthropic-beta", "mcp-client-2025-04-04")
                .header("content-type", "application/json")
                .body(Body::from(
                    json!({
                        "model": "claude-test",
                        "max_tokens": 64,
                        "messages": [{
                            "role": "user",
                            "content": [
                                {"type": "text", "text": "hello", "cache_control": {"type": "ephemeral"}},
                                {"type": "image", "source": {"type": "url", "url": "https://example.com/a.png"}, "cache_control": {"type": "ephemeral"}}
                            ]
                        }],
                        "system": [{"type": "text", "text": "sys"}],
                        "thinking": {"type": "enabled", "budget_tokens": 1024},
                        "mcp_servers": [{"type": "url", "name": "mcp", "url": "https://example.com/mcp"}]
                    })
                    .to_string(),
                ))?,
        )
        .await?;

    assert_eq!(response.status(), StatusCode::OK);
    let captured = upstream.take();
    assert_eq!(captured.x_api_key.as_deref(), Some("sk-upstream-key-0000"));
    assert_eq!(captured.anthropic_version.as_deref(), Some("2023-06-01"));
    assert_eq!(
        captured.anthropic_beta.as_deref(),
        Some("mcp-client-2025-04-04")
    );
    assert_eq!(
        captured.body["system"],
        json!([{"type": "text", "text": "sys"}])
    );
    assert_eq!(captured.body["thinking"]["type"], json!("enabled"));
    assert_eq!(captured.body["mcp_servers"][0]["name"], json!("mcp"));
    assert_eq!(
        captured.body["messages"][0]["content"][1],
        json!({"type": "text", "text": "[image omitted]", "cache_control": {"type": "ephemeral"}})
    );
    Ok(())
}

#[derive(Clone, Default)]
struct CapturedRequest {
    body: Value,
    authorization: Option<String>,
    x_api_key: Option<String>,
    anthropic_version: Option<String>,
    anthropic_beta: Option<String>,
}

struct FakeUpstream {
    addr: std::net::SocketAddr,
    captured: Arc<Mutex<Option<CapturedRequest>>>,
}

impl FakeUpstream {
    async fn start_openai() -> Result<Self, Box<dyn std::error::Error>> {
        let captured = Arc::new(Mutex::new(None));
        let app = Router::new()
            .route("/chat/completions", post(openai_upstream))
            .with_state(captured.clone());
        Self::serve(app, captured).await
    }

    async fn start_claude() -> Result<Self, Box<dyn std::error::Error>> {
        let captured = Arc::new(Mutex::new(None));
        let app = Router::new()
            .route("/messages", post(claude_upstream))
            .with_state(captured.clone());
        Self::serve(app, captured).await
    }

    async fn serve(
        app: Router,
        captured: Arc<Mutex<Option<CapturedRequest>>>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let listener = TcpListener::bind("127.0.0.1:0").await?;
        let addr = listener.local_addr()?;

        tokio::spawn(async move {
            axum::serve(listener, app).await.unwrap();
        });

        Ok(Self { addr, captured })
    }

    fn base_url(&self) -> String {
        format!("http://{}", self.addr)
    }

    fn take(&self) -> CapturedRequest {
        self.captured.lock().unwrap().take().unwrap()
    }
}

async fn openai_upstream(
    State(captured): State<Arc<Mutex<Option<CapturedRequest>>>>,
    headers: HeaderMap,
    body: Bytes,
) -> impl IntoResponse {
    capture(captured, headers, body).await;
    Json(json!({
        "id": "chatcmpl-test",
        "object": "chat.completion",
        "choices": [{"index": 0, "message": {"role": "assistant", "content": "ok"}, "finish_reason": "stop"}]
    }))
}

async fn claude_upstream(
    State(captured): State<Arc<Mutex<Option<CapturedRequest>>>>,
    headers: HeaderMap,
    body: Bytes,
) -> impl IntoResponse {
    capture(captured, headers, body).await;
    Json(json!({
        "type": "message",
        "id": "msg-test",
        "role": "assistant",
        "content": [{"type": "text", "text": "ok"}],
        "stop_reason": "end_turn"
    }))
}

async fn capture(captured: Arc<Mutex<Option<CapturedRequest>>>, headers: HeaderMap, body: Bytes) {
    let body: Value = serde_json::from_slice(&body).unwrap();
    let req = CapturedRequest {
        body,
        authorization: header_value(&headers, "authorization"),
        x_api_key: header_value(&headers, "x-api-key"),
        anthropic_version: header_value(&headers, "anthropic-version"),
        anthropic_beta: header_value(&headers, "anthropic-beta"),
    };
    *captured.lock().unwrap() = Some(req);
}

fn header_value(headers: &HeaderMap, name: &'static str) -> Option<String> {
    headers
        .get(name)
        .and_then(|value| value.to_str().ok())
        .map(str::to_string)
}

fn handle(
    base_url: String,
    image_filter: ImageFilterConfig,
) -> Result<KeyPoolHandle, Box<dyn std::error::Error>> {
    Ok(KeyPoolHandle::try_new(
        KeyPool::new(HashMap::from([workspace()])),
        Config {
            fixed: FixedConfig {
                listen: "127.0.0.1:0".to_string(),
            },
            runtime: RuntimeConfig {
                accounts: Vec::new(),
                refresh_interval_secs: 3600,
                max_retries: 0,
                go: GoConfig {
                    base_url,
                    connect_timeout_secs: 5,
                    request_timeout_secs: 5,
                },
                image_filter,
                api_token: "admin-token".to_string(),
            },
        },
        ConfigStore::new("/tmp/oc-go-switch-proxy-test-config.yaml"),
        Arc::new(LogStore::new()),
    )?)
}

fn workspace() -> (String, WorkspacePool) {
    let id = "acct/workspace".to_string();
    (
        id.clone(),
        WorkspacePool {
            id,
            name: "workspace".to_string(),
            account_name: "acct".to_string(),
            account_label: "Account".to_string(),
            status: WorkspacePoolStatus::Available,
            plan: Some(SubscriptionPlan::Go),
            go_usage: Some(GoUsage {
                hourly_percent: 1,
                hourly_reset_sec: 3600,
                weekly_percent: 1,
                weekly_reset_sec: 3600,
                monthly_percent: 1,
                monthly_reset_sec: 3600,
            }),
            keys: VecDeque::from([PoolKey {
                id: "acct/workspace/key".to_string(),
                key_value: "sk-upstream-key-0000".to_string(),
                key_name: "key".to_string(),
            }]),
        },
    )
}
