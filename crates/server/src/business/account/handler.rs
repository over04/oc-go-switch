use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
};
use tracing::info;

use crate::{
    business::{
        account::{
            dto::{
                add::AccountAddReqDto,
                edit::AccountEditReqDto,
                list::{AccountListEntryDto, AccountListRespDto},
            },
            service::account_entry,
        },
        configuration::service::save_runtime_config,
        workspace::handle::KeyPoolHandle,
    },
    common::config::account::AccountConfig,
};

pub async fn list_accounts(State(handle): State<KeyPoolHandle>) -> Json<AccountListRespDto> {
    let config = handle.runtime_config();
    let accounts: Vec<AccountListEntryDto> = config.accounts.iter().map(account_entry).collect();
    Json(AccountListRespDto { accounts })
}

pub async fn add_account(
    State(handle): State<KeyPoolHandle>,
    Json(req): Json<AccountAddReqDto>,
) -> Result<Json<AccountListRespDto>, StatusCode> {
    if req.name.is_empty() || req.auth.is_empty() || req.label.is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }
    if req.name.len() > 128 || req.label.len() > 256 || req.auth.len() > 4096 {
        return Err(StatusCode::BAD_REQUEST);
    }

    let mut config = handle.runtime_config().as_ref().clone();
    if config
        .accounts
        .iter()
        .any(|account| account.name == req.name)
    {
        return Err(StatusCode::CONFLICT);
    }
    config.accounts.push(AccountConfig {
        name: req.name.clone(),
        auth: req.auth.clone(),
        label: req.label.clone(),
    });
    save_runtime_config(&handle, config).await?;
    handle.request_refresh();

    info!("已添加账户 '{}'", req.name);
    Ok(list_accounts(State(handle)).await)
}

pub async fn delete_account(
    State(handle): State<KeyPoolHandle>,
    Path(name): Path<String>,
) -> Result<Json<AccountListRespDto>, StatusCode> {
    let mut config = handle.runtime_config().as_ref().clone();
    let len_before = config.accounts.len();
    config.accounts.retain(|account| account.name != name);
    if config.accounts.len() == len_before {
        return Err(StatusCode::NOT_FOUND);
    }
    save_runtime_config(&handle, config).await?;
    handle.request_refresh();

    info!("已删除账户 '{}'", name);
    Ok(list_accounts(State(handle)).await)
}

pub async fn edit_account(
    State(handle): State<KeyPoolHandle>,
    Path(name): Path<String>,
    Json(req): Json<AccountEditReqDto>,
) -> Result<Json<AccountListRespDto>, StatusCode> {
    let mut config = handle.runtime_config().as_ref().clone();
    let account = config
        .accounts
        .iter_mut()
        .find(|account| account.name == name)
        .ok_or(StatusCode::NOT_FOUND)?;
    if let Some(ref auth) = req.auth {
        if !auth.is_empty() {
            account.auth = auth.clone();
        }
    }
    if let Some(ref label) = req.label {
        if !label.is_empty() {
            account.label = label.clone();
        }
    }
    save_runtime_config(&handle, config).await?;
    handle.request_refresh();

    info!("已编辑账户 '{}'", name);
    Ok(list_accounts(State(handle)).await)
}
