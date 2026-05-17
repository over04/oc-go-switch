use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
};
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::config::AccountConfig;
use crate::pool::pool::KeyPoolHandle;

#[derive(Debug, Deserialize)]
pub struct AddAccountRequest {
    pub name: String,
    pub auth: String,
    pub label: String,
}

#[derive(Debug, Serialize)]
pub struct AccountListEntry {
    pub name: String,
    pub label: String,
    pub auth_masked: String,
}

#[derive(Debug, Serialize)]
pub struct AccountListResponse {
    pub accounts: Vec<AccountListEntry>,
}

/// GET /api/accounts — 列出已配置的账户（auth 脱敏显示）
pub async fn list_accounts(State(handle): State<KeyPoolHandle>) -> Json<AccountListResponse> {
    let config = handle.config_snapshot().await;
    let accounts: Vec<AccountListEntry> = config
        .accounts
        .iter()
        .map(|a| AccountListEntry {
            name: a.name.clone(),
            label: a.label.clone(),
            auth_masked: mask_auth(&a.auth),
        })
        .collect();
    Json(AccountListResponse { accounts })
}

/// POST /api/accounts — 添加新账户
pub async fn add_account(
    State(handle): State<KeyPoolHandle>,
    Json(req): Json<AddAccountRequest>,
) -> Result<Json<AccountListResponse>, StatusCode> {
    if req.name.is_empty() || req.auth.is_empty() || req.label.is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }
    if req.name.len() > 128 || req.label.len() > 256 || req.auth.len() > 4096 {
        return Err(StatusCode::BAD_REQUEST);
    }

    let mut config = handle.config_snapshot().await;
    if config.accounts.iter().any(|a| a.name == req.name) {
        return Err(StatusCode::CONFLICT);
    }
    config.accounts.push(AccountConfig {
        name: req.name.clone(),
        auth: req.auth.clone(),
        label: req.label.clone(),
    });
    save_config(&handle, config).await?;
    handle.request_refresh();

    info!("已添加账户 '{}'", req.name);
    Ok(list_accounts(State(handle)).await)
}

/// DELETE /api/accounts/{name} — 按名称删除账户
pub async fn delete_account(
    State(handle): State<KeyPoolHandle>,
    Path(name): Path<String>,
) -> Result<Json<AccountListResponse>, StatusCode> {
    let mut config = handle.config_snapshot().await;
    let len_before = config.accounts.len();
    config.accounts.retain(|a| a.name != name);
    if config.accounts.len() == len_before {
        return Err(StatusCode::NOT_FOUND);
    }
    save_config(&handle, config).await?;
    handle.request_refresh();

    info!("已删除账户 '{}'", name);
    Ok(list_accounts(State(handle)).await)
}

/// POST /api/pool/refresh — 强制触发重新发现
pub async fn force_refresh(
    State(handle): State<KeyPoolHandle>,
) -> Result<&'static str, StatusCode> {
    match handle.refresh_now().await {
        Ok(true) => {
            info!("强制刷新完成");
            Ok("ok")
        }
        Ok(false) => Ok("refresh already running"),
        Err(e) => {
            tracing::error!("强制刷新失败: {e}");
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

// ── 完整配置管理 ──────────────────────────────────────────

#[derive(Debug, Serialize)]
pub struct ConfigResponse {
    pub listen: String,
    pub refresh_interval_secs: u64,
    pub max_retries: usize,
    pub go: crate::config::GoConfig,
    pub accounts: Vec<AccountListEntry>,
    pub image_filter: crate::config::ImageFilterConfig,
    /// 是否已配置 API token（不返回 token 原文）。
    pub api_token_set: bool,
}

#[derive(Debug, Deserialize)]
pub struct UpdateConfigRequest {
    pub refresh_interval_secs: Option<u64>,
    pub max_retries: Option<usize>,
    pub image_filter: Option<crate::config::ImageFilterConfig>,
    pub api_token: Option<String>,
}

/// GET /api/config — 获取完整配置（auth 脱敏）
pub async fn get_config(State(handle): State<KeyPoolHandle>) -> Json<ConfigResponse> {
    let config = handle.config_snapshot().await;
    Json(ConfigResponse {
        listen: config.listen.clone(),
        refresh_interval_secs: config.refresh_interval_secs,
        max_retries: config.max_retries,
        go: config.go.clone(),
        accounts: config
            .accounts
            .iter()
            .map(|a| AccountListEntry {
                name: a.name.clone(),
                label: a.label.clone(),
                auth_masked: mask_auth(&a.auth),
            })
            .collect(),
        image_filter: config.image_filter.clone(),
        api_token_set: !config.api_token.trim().is_empty(),
    })
}

/// PUT /api/config — 更新配置
pub async fn update_config(
    State(handle): State<KeyPoolHandle>,
    Json(req): Json<UpdateConfigRequest>,
) -> Result<Json<ConfigResponse>, StatusCode> {
    let mut config = handle.config_snapshot().await;
    if let Some(v) = req.refresh_interval_secs {
        config.refresh_interval_secs = v;
    }
    if let Some(v) = req.max_retries {
        if v > 100 {
            return Err(StatusCode::BAD_REQUEST);
        }
        config.max_retries = v;
    }
    if let Some(ref v) = req.image_filter {
        config.image_filter = v.clone();
    }
    if let Some(v) = req.api_token {
        config.api_token = validate_token_input(v)?;
    }
    info!("update_config: 准备保存配置...");
    save_config(&handle, config).await?;
    info!("update_config: 配置已保存");
    info!("配置已更新");
    Ok(get_config(State(handle)).await)
}

// ── 活跃 key 管理 ───────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct SetActiveKeyRequest {
    pub key_id: String,
}

/// PUT /api/pool/active-key — 手动设置活跃/sticky key
pub async fn set_active_key(
    State(handle): State<KeyPoolHandle>,
    Json(req): Json<SetActiveKeyRequest>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    if !handle.set_active_key(req.key_id).await {
        return Err(StatusCode::NOT_FOUND);
    }
    info!("已手动设置活跃 key");
    Ok(Json(serde_json::json!({"status": "ok"})))
}

/// DELETE /api/pool/active-key — 清除活跃 key（恢复自动选择）
pub async fn clear_active_key(State(handle): State<KeyPoolHandle>) -> Json<serde_json::Value> {
    handle.clear_active_key().await;
    info!("已清除活跃 key");
    Json(serde_json::json!({"status": "ok"}))
}

// ── 账户编辑 ─────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct EditAccountRequest {
    pub auth: Option<String>,
    pub label: Option<String>,
}

/// PUT /api/accounts/{name} — 编辑账户的标签或 auth
pub async fn edit_account(
    State(handle): State<KeyPoolHandle>,
    Path(name): Path<String>,
    Json(req): Json<EditAccountRequest>,
) -> Result<Json<AccountListResponse>, StatusCode> {
    let mut config = handle.config_snapshot().await;
    let acct = config
        .accounts
        .iter_mut()
        .find(|a| a.name == name)
        .ok_or(StatusCode::NOT_FOUND)?;
    if let Some(ref new_auth) = req.auth {
        if !new_auth.is_empty() {
            acct.auth = new_auth.clone();
        }
    }
    if let Some(ref new_label) = req.label {
        if !new_label.is_empty() {
            acct.label = new_label.clone();
        }
    }
    save_config(&handle, config).await?;
    handle.request_refresh();

    info!("已编辑账户 '{}'", name);
    Ok(list_accounts(State(handle)).await)
}

fn mask_auth(auth: &str) -> String {
    if auth.len() <= 12 {
        return "***".to_string();
    }
    format!("{}...{}", &auth[..6], &auth[auth.len() - 4..])
}

fn validate_token_input(value: String) -> Result<String, StatusCode> {
    if value.trim().is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }
    Ok(value)
}

async fn save_config(
    handle: &KeyPoolHandle,
    config: crate::config::Config,
) -> Result<(), StatusCode> {
    handle.save_config_snapshot(config).await.map_err(|e| {
        tracing::error!("保存配置失败: {e}");
        StatusCode::INTERNAL_SERVER_ERROR
    })
}
