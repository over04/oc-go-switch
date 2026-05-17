use axum::{extract::State, http::StatusCode, response::Json};
use tracing::info;

use crate::business::{
    account::service::{account_entry, save_config},
    configuration::{
        dto::{get::ConfigurationGetRespDto, update::ConfigurationUpdateReqDto},
        service::validate_token_input,
    },
    workspace::handle::KeyPoolHandle,
};

pub async fn get_config(State(handle): State<KeyPoolHandle>) -> Json<ConfigurationGetRespDto> {
    let config = handle.config_snapshot();
    Json(ConfigurationGetRespDto {
        listen: config.listen.clone(),
        refresh_interval_secs: config.refresh_interval_secs,
        max_retries: config.max_retries,
        go: config.go.clone(),
        accounts: config.accounts.iter().map(account_entry).collect(),
        image_filter: config.image_filter.clone(),
        api_token_set: !config.api_token.trim().is_empty(),
    })
}

pub async fn update_config(
    State(handle): State<KeyPoolHandle>,
    Json(req): Json<ConfigurationUpdateReqDto>,
) -> Result<Json<ConfigurationGetRespDto>, StatusCode> {
    let mut config = handle.config_snapshot().as_ref().clone();
    if let Some(value) = req.refresh_interval_secs {
        config.refresh_interval_secs = value;
    }
    if let Some(value) = req.max_retries {
        if value > 100 {
            return Err(StatusCode::BAD_REQUEST);
        }
        config.max_retries = value;
    }
    if let Some(value) = req.go {
        config.go = value;
    }
    if let Some(ref value) = req.image_filter {
        config.image_filter = value.clone();
    }
    if let Some(value) = req.api_token {
        config.api_token = validate_token_input(value)?;
    }
    save_config(&handle, config).await?;
    info!("配置已更新");
    Ok(get_config(State(handle)).await)
}
