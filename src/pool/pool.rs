use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use chrono::Utc;
use tokio::sync::RwLock;
use tracing::{error, info, warn};

use crate::api::logs::LogStore;
use crate::config::Config;
use crate::model::LogEntry;
use crate::opencode::client::OpencodeClient;
use crate::opencode::types::SubscriptionPlan;

use super::key::PoolKey;
use super::selector::StickyKeySelector;

/// Pick the best key ID from the Go-only pool.
/// Rules:
/// 1. Only Go-subscribed, non-depleted keys with positive balance
/// 2. At most 1 key per workspace (highest balance)
/// 3. Sort by balance desc
fn pick_best_key_id(keys: &[PoolKey], depleted_ids: Option<&HashSet<String>>) -> Option<String> {
    let excluded = |id: &str| depleted_ids.is_some_and(|s| s.contains(id));
    let mut best_per_ws: HashMap<&str, &PoolKey> = HashMap::new();
    for k in keys.iter().filter(|k| {
        !k.depleted && k.subscribed && k.balance_cents > 0 && !excluded(&k.id)
    }) {
        let entry = best_per_ws.entry(&k.workspace_id).or_insert(k);
        if k.balance_cents > entry.balance_cents {
            *entry = k;
        }
    }

    let mut candidates: Vec<&&PoolKey> = best_per_ws.values().collect();
    if candidates.is_empty() {
        return None;
    }

    candidates.sort_by(|a, b| b.balance_cents.cmp(&a.balance_cents));

    Some(candidates[0].id.clone())
}

#[derive(Debug)]
pub struct KeyPool {
    pub keys: Vec<PoolKey>,
    pub selector: StickyKeySelector,
    pub last_refresh_at: Option<String>,
}

#[derive(Debug, Clone)]
pub struct KeyPoolHandle {
    pub(crate) inner: Arc<RwLock<KeyPool>>,
    pub(crate) config: Arc<RwLock<Config>>,
    pub log_store: Arc<LogStore>,
    pub http_client: reqwest::Client,
}

impl KeyPoolHandle {
    pub async fn read(&self) -> tokio::sync::RwLockReadGuard<'_, KeyPool> {
        self.inner.read().await
    }

    pub fn config(&self) -> Arc<RwLock<Config>> {
        self.config.clone()
    }
}

/// Discover all keys across all configured accounts and build a KeyPool.
pub async fn discover(config: &Config) -> anyhow::Result<KeyPool> {
    let mut all_keys: Vec<PoolKey> = Vec::new();

    for account in &config.accounts {
        info!(
            "Discovering workspaces for account '{}' ({})",
            account.name, account.label
        );

        let oc = OpencodeClient::new(&account.name, &account.auth);

        let workspaces = match oc.get_workspaces().await {
            Ok(ws) => ws,
            Err(e) => {
                error!(
                    "Failed to get workspaces for account '{}': {e}",
                    account.name
                );
                continue;
            }
        };

        info!(
            "Account '{}': found {} workspace(s)",
            account.name,
            workspaces.len()
        );

        for ws in &workspaces {
            let keys = match oc.list_keys(&ws.id).await {
                Ok(k) => k,
                Err(e) => {
                    warn!(
                        "Failed to list keys for workspace '{}' ({}/{}): {e}",
                        ws.name, account.name, ws.id
                    );
                    continue;
                }
            };

            let billing = oc.get_billing(&ws.id).await.ok();
            let go_usage = match &billing {
                Some(b) if b.subscribed => match oc.get_go_usage(&ws.id).await {
                    Ok(u) => u,
                    Err(e) => {
                        warn!("Failed to get Go usage for workspace '{}': {e}", ws.name);
                        None
                    }
                },
                _ => None,
            };

            let subscribed = billing
                .as_ref()
                .map(|b| b.subscribed && b.plan == Some(SubscriptionPlan::Go))
                .unwrap_or(false);

            let balance_raw = billing.as_ref().map(|b| b.balance).unwrap_or(0);

            // Skip Zen-subscribed workspaces
            if let Some(ref b) = billing {
                if b.plan == Some(SubscriptionPlan::Zen) {
                    info!(
                        "Skipping Zen workspace '{}' (account '{}')",
                        ws.name, account.name
                    );
                    continue;
                }
            }

            // Only Go-subscribed workspaces enter the pool
            if !subscribed {
                info!(
                    "Skipping non-Go workspace '{}' (account '{}')",
                    ws.name, account.name
                );
                continue;
            }

            for key_entry in &keys {
                let pool_key = PoolKey {
                    id: format!("{}/{}/{}", account.name, ws.id, key_entry.id),
                    account_name: account.name.clone(),
                    account_label: account.label.clone(),
                    workspace_id: ws.id.clone(),
                    workspace_name: ws.name.clone(),
                    key_value: key_entry.key.clone(),
                    key_name: key_entry.name.clone(),
                    balance_cents: balance_raw,
                    plan: billing.as_ref().and_then(|b| b.plan),
                    subscribed,
                    depleted: false,
                    go_usage: go_usage.clone(),
                };

                info!(
                    "Found key: {} | workspace={} subscribed=true go_usage={}",
                    pool_key.masked_key(),
                    pool_key.workspace_name,
                    go_usage.is_some(),
                );

                all_keys.push(pool_key);
            }
        }
    }

    if all_keys.is_empty() {
        return Err(anyhow::anyhow!(
            "No Go-subscribed API keys discovered. Check config and account auth."
        ));
    }

    info!("KeyPool: total {} Go-subscribed key(s) discovered", all_keys.len());

    Ok(KeyPool {
        keys: all_keys,
        selector: StickyKeySelector::new(),
        last_refresh_at: None,
    })
}

pub fn make_handle(
    pool: KeyPool,
    config: Arc<RwLock<Config>>,
    log_store: Arc<LogStore>,
) -> KeyPoolHandle {
    let http_client = reqwest::Client::new();
    KeyPoolHandle {
        inner: Arc::new(RwLock::new(pool)),
        config,
        log_store,
        http_client,
    }
}

impl KeyPoolHandle {
    /// Select a key. Pass `depleted_ids` to skip keys already exhausted
    /// during this request; pass `None` for general use.
    pub async fn select_key(
        &self,
        depleted_ids: Option<&HashSet<String>>,
    ) -> Option<PoolKey> {
        let excluded = |id: &str| depleted_ids.is_some_and(|s| s.contains(id));

        {
            let pool = self.inner.read().await;
            let current_id = pool.selector.current_id().cloned();
            if let Some(ref id) = current_id {
                if let Some(key) = pool.keys.iter().find(|k| {
                    &k.id == id && !k.depleted && k.subscribed && !excluded(&k.id)
                }) {
                    return Some(key.clone());
                }
            }
        }

        if let Some(key) = self.pick_and_set_key(depleted_ids).await {
            return Some(key);
        }

        if self.trigger_refresh(depleted_ids).await {
            return self.pick_and_set_key(depleted_ids).await;
        }

        None
    }

    async fn pick_and_set_key(
        &self,
        depleted_ids: Option<&HashSet<String>>,
    ) -> Option<PoolKey> {
        let picked_id: Option<String> = {
            let pool = self.inner.read().await;
            pick_best_key_id(&pool.keys, depleted_ids)
        };

        if let Some(ref id) = picked_id {
            let mut pool = self.inner.write().await;
            pool.selector.set_current(id.clone());
            return pool.keys.iter().find(|k| &k.id == id).cloned();
        }

        None
    }

    /// Trigger an immediate pool refresh. Keys in `depleted_ids` are
    /// marked depleted after the new pool is loaded so they are not
    /// re-selected during the same request.
    pub async fn trigger_refresh(
        &self,
        depleted_ids: Option<&HashSet<String>>,
    ) -> bool {
        let config_guard = self.config.read().await;
        match discover(&config_guard).await {
            Ok(mut new_pool) => {
                drop(config_guard);
                if let Some(ids) = depleted_ids {
                    for k in &mut new_pool.keys {
                        if ids.contains(&k.id) {
                            k.depleted = true;
                        }
                    }
                }
                let best_id = pick_best_key_id(&new_pool.keys, depleted_ids);
                new_pool.selector = super::selector::StickyKeySelector::new();
                if let Some(ref id) = best_id {
                    new_pool.selector.set_current(id.clone());
                }
                let now = Utc::now().to_rfc3339();
                new_pool.last_refresh_at = Some(now);
                let count = new_pool.keys.len();
                let mut pool = self.inner.write().await;
                *pool = new_pool;
                info!(
                    "trigger_refresh: pool refreshed, {} key(s) available",
                    count
                );
                true
            }
            Err(e) => {
                error!("trigger_refresh: discover failed: {e}");
                false
            }
        }
    }

    /// Record a proxy request log entry.
    pub async fn record_log(&self, entry: LogEntry) {
        self.log_store.record(entry).await;
    }

    /// Mark the current sticky key as depleted, then spawn an async
    /// background task to refresh the balance for that workspace so
    /// the depleted flag stays accurate.
    pub async fn mark_current_depleted(&self) {
        let depleted_info = {
            let mut pool = self.inner.write().await;
            let current_id = pool.selector.current_id().cloned();
            let mut target: Option<(String, String, String)> = None;
            if let Some(ref id) = current_id {
                for k in &mut pool.keys {
                    if k.id == *id {
                        k.depleted = true;
                        target = Some((
                            k.account_name.clone(),
                            k.workspace_id.clone(),
                            k.key_value.clone(),
                        ));
                    }
                }
            }
            pool.selector.reset();
            target
        };

        if let Some((account_name, ws_id, key_value)) = depleted_info {
            let handle = self.clone();
            tokio::spawn(async move {
                if let Err(e) = handle
                    .refresh_workspace_balance(&account_name, &ws_id)
                    .await
                {
                    warn!(
                        "Background balance refresh failed for workspace {}/{}: {e}",
                        account_name, ws_id
                    );
                } else {
                    info!(
                        "Balance refreshed for depleted key {}",
                        PoolKey::mask_value(&key_value)
                    );
                }
            });
        }
    }

    /// Refresh balance and Go usage for a single workspace (lightweight —
    /// does not run full discovery). Updates all keys belonging to the
    /// same workspace in the pool.
    async fn refresh_workspace_balance(
        &self,
        account_name: &str,
        workspace_id: &str,
    ) -> Result<(), anyhow::Error> {
        let account = {
            let config = self.config.read().await;
            config
                .accounts
                .iter()
                .find(|a| a.name == account_name)
                .cloned()
                .ok_or_else(|| anyhow::anyhow!("Account '{account_name}' not found in config"))?
        };

        let oc = OpencodeClient::new(&account.name, &account.auth);
        let billing = oc.get_billing(workspace_id).await?;
        let go_usage = if billing.subscribed {
            oc.get_go_usage(workspace_id).await.unwrap_or_else(|e| {
                warn!("Failed to get Go usage for workspace '{workspace_id}': {e}");
                None
            })
        } else {
            None
        };

        let balance_raw = billing.balance;

        let mut pool = self.inner.write().await;
        for k in &mut pool.keys {
            if k.workspace_id == workspace_id && k.account_name == account_name {
                k.balance_cents = balance_raw;
                k.go_usage = go_usage.clone();
                // If the balance has recovered, lift the depleted flag.
                if balance_raw > 0 {
                    k.depleted = false;
                    info!("Key {} depleted flag cleared (balance recovered)", k.masked_key());
                }
            }
        }

        info!(
            "Workspace {workspace_id} balance refreshed: {balance_raw} cents",
        );
        Ok(())
    }
}
