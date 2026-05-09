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

/// GET /api/accounts — list configured accounts (with masked auth)
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

/// POST /api/accounts — add a new account
pub async fn add_account(
    State(handle): State<KeyPoolHandle>,
    Json(req): Json<AddAccountRequest>,
) -> Result<Json<AccountListResponse>, StatusCode> {
    if req.name.is_empty() || req.auth.is_empty() || req.label.is_empty() {
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

    // Trigger rediscovery in background
    let h = handle.clone();
    tokio::spawn(async move {
        let cfg_arc = h.config();
        let config_guard = cfg_arc.read().await;
        match discover(&config_guard).await {
            Ok(new_pool) => {
                drop(config_guard);
                let mut pool = h.inner.write().await;
                *pool = new_pool;
            }
            Err(e) => {
                tracing::error!("Rediscovery after account add failed: {e}");
            }
        }
    });

    info!("Account '{}' added", req.name);
    Ok(list_accounts(State(handle)).await)
}

/// DELETE /api/accounts/{name} — remove an account by name
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

    // Trigger rediscovery in background
    let h = handle.clone();
    tokio::spawn(async move {
        let cfg_arc = h.config();
        let config_guard = cfg_arc.read().await;
        match discover(&config_guard).await {
            Ok(new_pool) => {
                drop(config_guard);
                let mut pool = h.inner.write().await;
                *pool = new_pool;
            }
            Err(e) => {
                tracing::error!("Rediscovery after account delete failed: {e}");
            }
        }
    });

    info!("Account '{}' deleted", name);
    Ok(list_accounts(State(handle)).await)
}

/// POST /api/pool/refresh — force trigger rediscovery
pub async fn force_refresh(State(handle): State<KeyPoolHandle>) -> Result<&'static str, StatusCode> {
    let h = handle.clone();
    let cfg_arc = h.config();
    let config_guard = cfg_arc.read().await;
    match discover(&config_guard).await {
        Ok(new_pool) => {
            let count = new_pool.keys.len();
            drop(config_guard);
            let mut pool = h.inner.write().await;
            *pool = new_pool;
            info!("Force refresh complete: {} keys", count);
            Ok("ok")
        }
        Err(e) => {
            tracing::error!("Force refresh failed: {e}");
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

// ── Full config management ──────────────────────────────────────────

#[derive(Debug, Serialize)]
pub struct ConfigResponse {
    pub listen: String,
    pub refresh_interval_secs: u64,
    pub max_retries: usize,
    pub go: crate::config::GoConfig,
    pub accounts: Vec<AccountListEntry>,
    pub image_filter: crate::config::ImageFilterConfig,
}

#[derive(Debug, Deserialize)]
pub struct UpdateConfigRequest {
    pub refresh_interval_secs: Option<u64>,
    pub max_retries: Option<usize>,
    pub image_filter: Option<crate::config::ImageFilterConfig>,
}

/// GET /api/config — full config with masked auth
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
    })
}

/// PUT /api/config — update config
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
            config.max_retries = v;
        }
        if let Some(ref v) = req.image_filter {
            config.image_filter = v.clone();
        }
        save_config(&config)?;
    }
    info!("Config updated");
    Ok(get_config(State(handle)).await)
}

// ── Active key management ───────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct SetActiveKeyRequest {
    pub key_id: String,
}

/// PUT /api/pool/active-key — manually set the active/sticky key
pub async fn set_active_key(
    State(handle): State<KeyPoolHandle>,
    Json(req): Json<SetActiveKeyRequest>,
) -> Result<&'static str, StatusCode> {
    let mut pool = handle.inner.write().await;
    // Verify key exists
    if !pool.keys.iter().any(|k| k.id == req.key_id && !k.depleted) {
        return Err(StatusCode::NOT_FOUND);
    }
    pool.selector.set_current(req.key_id);
    info!("Active key set manually");
    Ok("ok")
}

/// DELETE /api/pool/active-key — clear the active key (revert to auto-selection)
pub async fn clear_active_key(
    State(handle): State<KeyPoolHandle>,
) -> &'static str {
    let mut pool = handle.inner.write().await;
    pool.selector.reset();
    info!("Active key cleared");
    "ok"
}

// ── Account editing ─────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct EditAccountRequest {
    pub auth: Option<String>,
    pub label: Option<String>,
}

/// PUT /api/accounts/{name} — edit an account's label or auth
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
        // Trigger rediscovery
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

    info!("Account '{}' edited", name);
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
        tracing::error!("Failed to serialize config: {e}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    std::fs::write("config.yaml", yaml).map_err(|e| {
        tracing::error!("Failed to write config.yaml: {e}");
        StatusCode::INTERNAL_SERVER_ERROR
    })
}
