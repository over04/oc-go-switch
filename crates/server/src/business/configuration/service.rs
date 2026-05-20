use axum::http::StatusCode;

use crate::{
    business::{
        configuration::dto::update::ConfigurationUpdateReqDto, workspace::handle::KeyPoolHandle,
    },
    common::config::runtime::RuntimeConfig,
};

pub fn validate_token_input(value: String) -> Result<String, StatusCode> {
    if value.trim().is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }
    Ok(value)
}

pub async fn save_runtime_patch(
    handle: &KeyPoolHandle,
    req: ConfigurationUpdateReqDto,
) -> Result<(), StatusCode> {
    let mut runtime = handle.runtime_config().as_ref().clone();
    let Some(patch) = req.runtime else {
        return Ok(());
    };

    if let Some(value) = patch.refresh_interval_secs {
        runtime.refresh_interval_secs = value;
    }
    if let Some(value) = patch.max_retries {
        if value > 100 {
            return Err(StatusCode::BAD_REQUEST);
        }
        runtime.max_retries = value;
    }
    if let Some(value) = patch.go {
        runtime.go = value;
    }
    if let Some(value) = patch.image_filter {
        runtime.image_filter = value;
    }
    if let Some(value) = patch.api_token {
        runtime.api_token = validate_token_input(value)?;
    }

    save_runtime_config(handle, runtime).await
}

pub async fn save_runtime_config(
    handle: &KeyPoolHandle,
    config: RuntimeConfig,
) -> Result<(), StatusCode> {
    handle.save_runtime_config(config).await.map_err(|error| {
        tracing::error!("保存配置失败: {error}");
        StatusCode::INTERNAL_SERVER_ERROR
    })
}
