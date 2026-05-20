use axum::{extract::State, http::StatusCode, response::Json};
use tracing::info;

use crate::business::{
    account::service::account_entry,
    configuration::{
        dto::{
            get::{
                ConfigurationFixedRespDto, ConfigurationGetRespDto, ConfigurationRuntimeRespDto,
            },
            update::ConfigurationUpdateReqDto,
        },
        service::save_runtime_patch,
    },
    workspace::handle::KeyPoolHandle,
};

pub async fn get_config(State(handle): State<KeyPoolHandle>) -> Json<ConfigurationGetRespDto> {
    let fixed = handle.fixed_config();
    let runtime = handle.runtime_config();
    Json(ConfigurationGetRespDto {
        fixed: ConfigurationFixedRespDto {
            listen: fixed.listen.clone(),
        },
        runtime: ConfigurationRuntimeRespDto {
            accounts: runtime.accounts.iter().map(account_entry).collect(),
            refresh_interval_secs: runtime.refresh_interval_secs,
            max_retries: runtime.max_retries,
            go: runtime.go.clone(),
            image_filter: runtime.image_filter.clone(),
            api_token_set: !runtime.api_token.trim().is_empty(),
        },
    })
}

pub async fn update_config(
    State(handle): State<KeyPoolHandle>,
    Json(req): Json<ConfigurationUpdateReqDto>,
) -> Result<Json<ConfigurationGetRespDto>, StatusCode> {
    save_runtime_patch(&handle, req).await?;
    info!("配置已更新");
    Ok(get_config(State(handle)).await)
}
