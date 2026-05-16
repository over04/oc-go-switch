use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
};
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::config::{AccountConfig, Config};
use crate::pool::pool::{discover, KeyPoolHandle};

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
    let cfg = handle.config();
    let config = cfg.read().await;
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

    {
        let cfg = handle.config();
        let mut config = cfg.write().await;
        if config.accounts.iter().any(|a| a.name == req.name) {
            return Err(StatusCode::CONFLICT);
        }
        config.accounts.push(AccountConfig {
            name: req.name.clone(),
            auth: req.auth.clone(),
            label: req.label.clone(),
        });
        save_config(&config)?;
    }

    let h = handle.clone();
    tokio::spawn(async move {
        let old_sticky_id = {
            let pool = h.inner.read().await;
            pool.selector.current_id().cloned()
        };
        let cfg_arc = h.config();
        let config_guard = cfg_arc.read().await;
        match discover(&config_guard).await {
            Ok(mut new_pool) => {
                drop(config_guard);
                if let Some(ref old_id) = old_sticky_id {
                    let viable = new_pool.keys.iter().any(|k| {
                        &k.id == old_id && !k.depleted && k.subscribed && !k.is_fully_exhausted()
                    });
                    if viable {
                        new_pool.selector.set_current(old_id.clone());
                    }
                }
                let mut pool = h.inner.write().await;
                *pool = new_pool;
            }
            Err(e) => {
                tracing::error!("添加账户后重新发现失败: {e}");
            }
        }
    });

    info!("已添加账户 '{}'", req.name);
    Ok(list_accounts(State(handle)).await)
}

/// DELETE /api/accounts/{name} — 按名称删除账户
pub async fn delete_account(
    State(handle): State<KeyPoolHandle>,
    Path(name): Path<String>,
) -> Result<Json<AccountListResponse>, StatusCode> {
    {
        let cfg = handle.config();
        let mut config = cfg.write().await;
        let len_before = config.accounts.len();
        config.accounts.retain(|a| a.name != name);
        if config.accounts.len() == len_before {
            return Err(StatusCode::NOT_FOUND);
        }
        save_config(&config)?;
    }

    // 后台触发重新发现
    let h = handle.clone();
    tokio::spawn(async move {
        let old_sticky_id = {
            let pool = h.inner.read().await;
            pool.selector.current_id().cloned()
        };
        let cfg_arc = h.config();
        let config_guard = cfg_arc.read().await;
        match discover(&config_guard).await {
            Ok(mut new_pool) => {
                drop(config_guard);
                if let Some(ref old_id) = old_sticky_id {
                    let viable = new_pool.keys.iter().any(|k| {
                        &k.id == old_id && !k.depleted && k.subscribed && !k.is_fully_exhausted()
                    });
                    if viable {
                        new_pool.selector.set_current(old_id.clone());
                    }
                }
                let mut pool = h.inner.write().await;
                *pool = new_pool;
            }
            Err(e) => {
                tracing::error!("删除账户后重新发现失败: {e}");
            }
        }
    });

    info!("已删除账户 '{}'", name);
    Ok(list_accounts(State(handle)).await)
}

/// POST /api/pool/refresh — 强制触发重新发现
pub async fn force_refresh(
    State(handle): State<KeyPoolHandle>,
) -> Result<&'static str, StatusCode> {
    let old_sticky_id = {
        let pool = handle.inner.read().await;
        pool.selector.current_id().cloned()
    };
    let h = handle.clone();
    let cfg_arc = h.config();
    let config_guard = cfg_arc.read().await;
    match discover(&config_guard).await {
        Ok(mut new_pool) => {
            let count = new_pool.keys.len();
            drop(config_guard);
            if let Some(ref old_id) = old_sticky_id {
                let viable = new_pool.keys.iter().any(|k| {
                    &k.id == old_id && !k.depleted && k.subscribed && !k.is_fully_exhausted()
                });
                if viable {
                    new_pool.selector.set_current(old_id.clone());
                }
            }
            let mut pool = h.inner.write().await;
            *pool = new_pool;
            info!("强制刷新完成: {} 个 key", count);
            Ok("ok")
        }
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
    let cfg = handle.config();
    let config = cfg.read().await;
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
        api_token_set: config.api_token.is_some(),
    })
}

/// PUT /api/config — 更新配置
pub async fn update_config(
    State(handle): State<KeyPoolHandle>,
    Json(req): Json<UpdateConfigRequest>,
) -> Result<Json<ConfigResponse>, StatusCode> {
    {
        let cfg = handle.config();
        let mut config = cfg.write().await;
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
            config.api_token = if v.is_empty() { None } else { Some(v) };
        }
        info!("update_config: 准备保存配置...");
        save_config(&config)?;
        info!("update_config: 配置已保存");
    }
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
    let mut pool = handle.inner.write().await;
    if !pool
        .keys
        .iter()
        .any(|k| k.id == req.key_id && !k.depleted && !k.is_fully_exhausted())
    {
        return Err(StatusCode::NOT_FOUND);
    }
    pool.selector.set_current(req.key_id);
    info!("已手动设置活跃 key");
    Ok(Json(serde_json::json!({"status": "ok"})))
}

/// DELETE /api/pool/active-key — 清除活跃 key（恢复自动选择）
pub async fn clear_active_key(
    State(handle): State<KeyPoolHandle>,
) -> Json<serde_json::Value> {
    let mut pool = handle.inner.write().await;
    pool.selector.reset();
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
    let changed;
    {
        let cfg = handle.config();
        let mut config = cfg.write().await;
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
        save_config(&config)?;
        changed = config.accounts.iter().any(|a| a.name == name);
    }

    if changed {
        // 触发重新发现
        let h = handle.clone();
        tokio::spawn(async move {
            let cfg_arc = h.config();
            let config_guard = cfg_arc.read().await;
            if let Ok(new_pool) = discover(&config_guard).await {
                drop(config_guard);
                let mut pool = h.inner.write().await;
                *pool = new_pool;
            }
        });
    }

    info!("已编辑账户 '{}'", name);
    Ok(list_accounts(State(handle)).await)
}

fn mask_auth(auth: &str) -> String {
    if auth.len() <= 12 {
        return "***".to_string();
    }
    format!("{}...{}", &auth[..6], &auth[auth.len() - 4..])
}

fn save_config(config: &Config) -> Result<(), StatusCode> {
    let yaml = serde_yaml::to_string(config).map_err(|e| {
        tracing::error!("序列化配置失败: {e}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    let tmp = "config.yaml.tmp";
    std::fs::write(tmp, &yaml).map_err(|e| {
        tracing::error!("写入临时配置文件失败: {e}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    std::fs::rename(tmp, "config.yaml").map_err(|e| {
        tracing::error!("替换配置文件失败: {e}");
        StatusCode::INTERNAL_SERVER_ERROR
    })
}
