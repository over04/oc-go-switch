use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Json},
};
use serde::{Deserialize, Serialize};

use crate::pool::pool::KeyPoolHandle;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelListResponse {
    pub object: String,
    pub data: Vec<ModelInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelInfo {
    pub id: String,
    pub owned_by: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum ModelListResult {
    Ok(ModelListResponse),
    Error { error: String },
}

#[derive(Debug, Serialize)]
pub struct MergedModelsResponse {
    pub openai: ModelListResult,
    pub claude: ModelListResult,
}

async fn fetch_model_list(
    handle: &KeyPoolHandle,
    base_url: &str,
) -> Result<ModelListResponse, String> {
    let key = handle
        .select_key_or_refresh()
        .await
        .ok_or_else(|| "没有可用 Go 工作区".to_string())?;
    let url = format!("{}/models", base_url);

    let resp = handle
        .short_client
        .get(&url)
        .header("Authorization", format!("Bearer {}", key.key_value))
        .send()
        .await
        .map_err(|e| format!("上游不可达: {e}"))?;

    let status = resp.status();
    let body = resp.text().await.map_err(|e| format!("读取错误: {e}"))?;

    if !status.is_success() {
        return Err(format!("HTTP {}: {}", status.as_u16(), body));
    }

    serde_json::from_str(&body).map_err(|e| format!("JSON 解析错误: {e}"))
}

/// GET /v1/models — OpenAI 兼容的模型列表（对外暴露）
pub async fn list_models_v1(State(handle): State<KeyPoolHandle>) -> impl IntoResponse {
    let base_url = handle.config_snapshot().await.go.base_url;

    match fetch_model_list(&handle, &base_url).await {
        Ok(r) => match serde_json::to_value(r) {
            Ok(v) => (StatusCode::OK, Json(v)).into_response(),
            Err(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": { "message": format!("序列化错误: {e}"), "type": "server_error" }
                })),
            )
                .into_response(),
        },
        Err(e) => (
            StatusCode::BAD_GATEWAY,
            Json(serde_json::json!({
                "error": { "message": e, "type": "proxy_error" }
            })),
        )
            .into_response(),
    }
}

/// GET /api/models — 仪表盘合并视图
pub async fn list_models(State(handle): State<KeyPoolHandle>) -> Json<MergedModelsResponse> {
    let base_url = handle.config_snapshot().await.go.base_url;

    let (openai, claude) = tokio::join!(
        fetch_model_list(&handle, &base_url),
        fetch_model_list(&handle, &base_url),
    );

    Json(MergedModelsResponse {
        openai: match openai {
            Ok(r) => ModelListResult::Ok(r),
            Err(e) => ModelListResult::Error { error: e },
        },
        claude: match claude {
            Ok(r) => ModelListResult::Ok(r),
            Err(e) => ModelListResult::Error { error: e },
        },
    })
}

/// GET /api/models/openai
pub async fn list_openai_models(State(handle): State<KeyPoolHandle>) -> Json<ModelListResult> {
    let url = handle.config_snapshot().await.go.base_url;

    Json(match fetch_model_list(&handle, &url).await {
        Ok(r) => ModelListResult::Ok(r),
        Err(e) => ModelListResult::Error { error: e },
    })
}

/// GET /api/models/claude
pub async fn list_claude_models(State(handle): State<KeyPoolHandle>) -> Json<ModelListResult> {
    let url = handle.config_snapshot().await.go.base_url;

    Json(match fetch_model_list(&handle, &url).await {
        Ok(r) => ModelListResult::Ok(r),
        Err(e) => ModelListResult::Error { error: e },
    })
}
