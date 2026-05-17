use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Json},
};

use crate::business::{
    model_catalog::{
        merged_models::MergedModelsResponse, model_list_result::ModelListResult,
        service::fetch_model_list,
    },
    workspace::handle::KeyPoolHandle,
};

pub async fn list_models_v1(State(handle): State<KeyPoolHandle>) -> impl IntoResponse {
    let base_url = handle.config_snapshot().go.base_url.clone();

    match fetch_model_list(&handle, &base_url).await {
        Ok(response) => (StatusCode::OK, Json(response)).into_response(),
        Err(error) => (
            StatusCode::BAD_GATEWAY,
            Json(serde_json::json!({
                "error": { "message": error, "type": "proxy_error" }
            })),
        )
            .into_response(),
    }
}

pub async fn list_models(State(handle): State<KeyPoolHandle>) -> Json<MergedModelsResponse> {
    let base_url = handle.config_snapshot().go.base_url.clone();

    let (openai, claude) = tokio::join!(
        fetch_model_list(&handle, &base_url),
        fetch_model_list(&handle, &base_url),
    );

    Json(MergedModelsResponse {
        openai: model_result(openai),
        claude: model_result(claude),
    })
}

pub async fn list_openai_models(State(handle): State<KeyPoolHandle>) -> Json<ModelListResult> {
    let url = handle.config_snapshot().go.base_url.clone();
    Json(model_result(fetch_model_list(&handle, &url).await))
}

pub async fn list_claude_models(State(handle): State<KeyPoolHandle>) -> Json<ModelListResult> {
    let url = handle.config_snapshot().go.base_url.clone();
    Json(model_result(fetch_model_list(&handle, &url).await))
}

fn model_result(
    result: Result<crate::business::model_catalog::model_list::ModelListResponse, String>,
) -> ModelListResult {
    match result {
        Ok(response) => ModelListResult::Ok(response),
        Err(error) => ModelListResult::Error { error },
    }
}
