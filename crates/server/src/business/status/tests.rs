use std::{collections::HashMap, sync::Arc};

use adapter::opencode::model::{go_usage::GoUsage, subscription_plan::SubscriptionPlan};
use axum::{
    body::{to_bytes, Body},
    http::{Request, StatusCode},
    Router,
};
use serde_json::{json, Value};
use tower::ServiceExt;

use crate::{
    business::{
        log::store::LogStore,
        status::router::{dashboard_router, workspace_router},
        workspace::{
            credential::WorkspaceCredential, handle::WorkspaceSchedulerHandle,
            record::WorkspacePool, scheduler::WorkspaceScheduler, status::WorkspacePoolStatus,
        },
    },
    common::config::{
        fixed::FixedConfig, go::GoConfig, image_filter::ImageFilterConfig, runtime::RuntimeConfig,
        store::ConfigStore, Config,
    },
};

#[tokio::test]
async fn workspace_status_exposes_workspace_level_schedule(
) -> Result<(), Box<dyn std::error::Error>> {
    let app = Router::new()
        .nest("/workspaces", workspace_router())
        .with_state(handle()?);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/workspaces/status")
                .body(Body::empty())?,
        )
        .await?;

    assert_eq!(response.status(), StatusCode::OK);
    let body = json_body(response).await?;

    assert_eq!(body["current_workspace_id"], Value::Null);
    assert_eq!(body["accounts"][0]["name"], "acct");
    assert_eq!(
        body["accounts"][0]["workspaces"][0]["id"],
        "acct/workspace-a"
    );
    assert_eq!(
        body["accounts"][0]["workspaces"][0]["credential_masked"],
        "sk-wor...0000"
    );
    assert!(body["accounts"][0]["workspaces"][0]
        .as_object()
        .is_some_and(|workspace| workspace.contains_key("credential_masked")));
    Ok(())
}

#[tokio::test]
async fn current_endpoint_accepts_plain_workspace_id() -> Result<(), Box<dyn std::error::Error>> {
    let app = Router::new()
        .nest("/workspaces", workspace_router())
        .with_state(handle()?);

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("PUT")
                .uri("/workspaces/current")
                .header("content-type", "application/json")
                .body(Body::from(
                    json!({"workspace_id": "acct/workspace-b"}).to_string(),
                ))?,
        )
        .await?;

    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(json_body(response).await?["status"], "ok");

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/workspaces/status")
                .body(Body::empty())?,
        )
        .await?;
    let body = json_body(response).await?;

    assert_eq!(body["current_workspace_id"], "acct/workspace-b");
    let workspace_b = body["accounts"][0]["workspaces"]
        .as_array()
        .and_then(|items| items.iter().find(|item| item["id"] == "acct/workspace-b"))
        .expect("workspace-b should be present");
    assert_eq!(workspace_b["is_current"], true);

    let response = app
        .oneshot(
            Request::builder()
                .method("DELETE")
                .uri("/workspaces/current")
                .body(Body::empty())?,
        )
        .await?;

    assert_eq!(response.status(), StatusCode::OK);
    Ok(())
}

#[tokio::test]
async fn dashboard_status_counts_workspaces_only() -> Result<(), Box<dyn std::error::Error>> {
    let app = Router::new()
        .nest("/dashboard", dashboard_router())
        .with_state(handle()?);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/dashboard/status")
                .body(Body::empty())?,
        )
        .await?;

    assert_eq!(response.status(), StatusCode::OK);
    let body = json_body(response).await?;
    assert_eq!(body["total_workspaces"], 2);
    assert_eq!(body["available_workspaces"], 2);
    assert_eq!(body["go_workspaces"].as_array().map(Vec::len), Some(2));
    Ok(())
}

async fn json_body(
    response: axum::response::Response,
) -> Result<Value, Box<dyn std::error::Error>> {
    let bytes = to_bytes(response.into_body(), usize::MAX).await?;
    Ok(serde_json::from_slice(&bytes)?)
}

fn handle() -> Result<WorkspaceSchedulerHandle, Box<dyn std::error::Error>> {
    Ok(WorkspaceSchedulerHandle::try_new(
        WorkspaceScheduler::new(HashMap::from([
            workspace("acct/workspace-a", 10, "sk-workspace-a-0000"),
            workspace("acct/workspace-b", 20, "sk-workspace-b-1111"),
        ])),
        Config {
            fixed: FixedConfig {
                listen: "127.0.0.1:0".to_string(),
            },
            runtime: RuntimeConfig {
                accounts: Vec::new(),
                refresh_interval_secs: 0,
                max_retries: 0,
                go: GoConfig {
                    base_url: "http://127.0.0.1:0".to_string(),
                    connect_timeout_secs: 1,
                    request_timeout_secs: 1,
                },
                image_filter: ImageFilterConfig::default(),
                api_token: "admin-token".to_string(),
            },
        },
        ConfigStore::new("/tmp/oc-go-switch-status-test-config.yaml"),
        Arc::new(LogStore::new()),
    )?)
}

fn workspace(id: &'static str, usage: u32, credential: &'static str) -> (String, WorkspacePool) {
    let id = id.to_string();
    (
        id.clone(),
        WorkspacePool {
            id: id.clone(),
            name: id.rsplit('/').next().unwrap_or(id.as_str()).to_string(),
            account_name: "acct".to_string(),
            account_label: "Account".to_string(),
            status: WorkspacePoolStatus::Available,
            plan: Some(SubscriptionPlan::Go),
            go_usage: Some(GoUsage {
                hourly_percent: usage,
                hourly_reset_sec: 3600,
                weekly_percent: usage,
                weekly_reset_sec: 3600,
                monthly_percent: usage,
                monthly_reset_sec: 3600,
            }),
            credential: WorkspaceCredential {
                value: credential.to_string(),
            },
        },
    )
}
