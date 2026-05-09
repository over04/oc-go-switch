use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::RwLock;
use tracing::{error, info, warn};

use crate::api::logs::LogStore;
use crate::config::Config;
use crate::model::LogEntry;
use crate::opencode::client::OpencodeClient;
use crate::opencode::types::SubscriptionPlan;

use crate::config::SortBy;

use super::key::PoolKey;
use super::selector::StickyKeySelector;

/// Pick the best key ID from the Go-only pool.
/// Rules:
/// 1. Only Go-subscribed, non-depleted keys
/// 2. At most 1 key per workspace (highest balance)
/// 3. Sort by balance desc
fn pick_best_key_id(keys: &[PoolKey], sort_by: SortBy) -> Option<String> {
    // Group by workspace_id, pick best (highest balance) per workspace
    let mut best_per_ws: HashMap<&str, &PoolKey> = HashMap::new();
    for k in keys.iter().filter(|k| !k.depleted && k.subscribed) {
        let entry = best_per_ws.entry(&k.workspace_id).or_insert(k);
        if k.balance_cents > entry.balance_cents {
            *entry = k;
        }
    }

    let mut candidates: Vec<&&PoolKey> = best_per_ws.values().collect();
    if candidates.is_empty() {
        return None;
    }

    candidates.sort_by(|a, b| match sort_by {
        SortBy::BalanceDesc => b.balance_cents.cmp(&a.balance_cents),
    });

    Some(candidates[0].id.clone())
}

#[derive(Debug)]
pub struct KeyPool {
    pub keys: Vec<PoolKey>,
    pub selector: StickyKeySelector,
}

#[derive(Debug, Clone)]
pub struct KeyPoolHandle {
    pub(crate) inner: Arc<RwLock<KeyPool>>,
    pub(crate) config: Arc<RwLock<Config>>,
    pub log_store: Arc<LogStore>,
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
    })
}

pub fn make_handle(
    pool: KeyPool,
    config: Arc<RwLock<Config>>,
    log_store: Arc<LogStore>,
) -> KeyPoolHandle {
    KeyPoolHandle {
        inner: Arc::new(RwLock::new(pool)),
        config,
        log_store,
    }
}

impl KeyPoolHandle {
    /// Select a key (sticky), returning a clone of the selected PoolKey.
    pub async fn select_key(&self) -> Option<PoolKey> {
        let sort_by = self.config.read().await.selection.sort_by;

        // First, check if the current sticky key is still valid (read lock)
        {
            let pool = self.inner.read().await;
            let current_id = pool.selector.current_id().cloned();
            if let Some(ref id) = current_id {
                if let Some(key) = pool.keys.iter().find(|k| &k.id == id && !k.depleted && k.subscribed) {
                    return Some(key.clone());
                }
            }
        }

        // Need to pick a new key
        let picked_id: Option<String> = {
            let pool = self.inner.read().await;
            pick_best_key_id(&pool.keys, sort_by)
        };

        if let Some(ref id) = picked_id {
            let mut pool = self.inner.write().await;
            pool.selector.set_current(id.clone());
            pool.keys.iter().find(|k| &k.id == id).cloned()
        } else {
            None
        }
    }

    /// Record a proxy request log entry.
    pub async fn record_log(&self, entry: LogEntry) {
        self.log_store.record(entry).await;
    }

    /// Mark the current sticky key as depleted.
    pub async fn mark_current_depleted(&self) {
        let mut pool = self.inner.write().await;
        let current_id = pool.selector.current_id().cloned();
        if let Some(id) = current_id {
            for k in &mut pool.keys {
                if k.id == id {
                    k.depleted = true;
                }
            }
        }
        pool.selector.reset();
    }
}
