use axum::http::StatusCode;

use crate::{
    business::{account::dto::list::AccountListEntryDto, workspace::handle::KeyPoolHandle},
    common::config::{account::AccountConfig, Config},
};

pub fn mask_auth(auth: &str) -> String {
    if auth.len() <= 12 {
        return "***".to_string();
    }
    format!("{}...{}", &auth[..6], &auth[auth.len() - 4..])
}

pub fn account_entry(account: &AccountConfig) -> AccountListEntryDto {
    AccountListEntryDto {
        name: account.name.clone(),
        label: account.label.clone(),
        auth_masked: mask_auth(&account.auth),
    }
}

pub async fn save_config(handle: &KeyPoolHandle, config: Config) -> Result<(), StatusCode> {
    handle.save_config_snapshot(config).await.map_err(|error| {
        tracing::error!("保存配置失败: {error}");
        StatusCode::INTERNAL_SERVER_ERROR
    })
}
