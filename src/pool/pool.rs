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

/// 从 Go 订阅池中选出最佳 key。
/// 规则：
/// 1. 只选已订阅 Go、未耗尽、未完全满额的 key
/// 2. 每个工作区最多选一个（用量最低的）
/// 3. 按用量百分比升序排列
fn pick_best_key_id(keys: &[PoolKey], depleted_ids: Option<&HashSet<String>>) -> Option<String> {
    let excluded = |id: &str| depleted_ids.is_some_and(|s| s.contains(id));
    let mut best_per_ws: HashMap<&str, &PoolKey> = HashMap::new();
    for k in keys.iter().filter(|k| {
        !k.depleted && k.subscribed && !k.is_fully_exhausted() && !excluded(&k.id)
    }) {
        let entry = best_per_ws.entry(&k.workspace_id).or_insert(k);
        if k.max_usage_pct() < entry.max_usage_pct() {
            *entry = k;
        }
    }

    let mut candidates: Vec<&&PoolKey> = best_per_ws.values().collect();
    if candidates.is_empty() {
        return None;
    }

    candidates.sort_by_key(|a| a.max_usage_pct());

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

/// 遍历所有配置的账户，发现工作区和 API key，构建 KeyPool。
pub async fn discover(config: &Config) -> anyhow::Result<KeyPool> {
    let mut all_keys: Vec<PoolKey> = Vec::new();

    for account in &config.accounts {
        info!(
            "正在发现账户 '{}' ({}) 的工作区...",
            account.name, account.label
        );

        let oc = OpencodeClient::new(&account.name, &account.auth);

        let workspaces = match oc.get_workspaces().await {
            Ok(ws) => ws,
            Err(e) => {
                error!("获取账户 '{}' 的工作区列表失败: {e}", account.name);
                continue;
            }
        };

        info!(
            "账户 '{}': 发现 {} 个工作区",
            account.name,
            workspaces.len()
        );

        for ws in &workspaces {
            let keys = match oc.list_keys(&ws.id).await {
                Ok(k) => k,
                Err(e) => {
                    warn!(
                        "获取工作区 '{}' ({}/{}) 的 key 列表失败: {e}",
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
                        warn!("获取工作区 '{}' 的 Go 用量失败: {e}", ws.name);
                        None
                    }
                },
                _ => None,
            };

            let subscribed = billing
                .as_ref()
                .map(|b| b.subscribed && b.plan == Some(SubscriptionPlan::Go))
                .unwrap_or(false);

            // 跳过 Zen 订阅的工作区
            if let Some(ref b) = billing {
                if b.plan == Some(SubscriptionPlan::Zen) {
                    info!(
                        "跳过 Zen 工作区 '{}' (账户 '{}')",
                        ws.name, account.name
                    );
                    continue;
                }
            }

            // 只有 Go 订阅的工作区才进入池
            if !subscribed {
                info!(
                    "跳过非 Go 工作区 '{}' (账户 '{}')",
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
                    plan: billing.as_ref().and_then(|b| b.plan),
                    subscribed,
                    depleted: false,
                    go_usage: go_usage.clone(),
                };

                info!(
                    "发现 key: {} | workspace={} subscribed=true go_usage={}",
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
            "未发现 Go 订阅的 API key，请检查配置和账户授权。"
        ));
    }

    info!("KeyPool: 共发现 {} 个 Go 订阅 key", all_keys.len());

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
    /// 选一个 key。`depleted_ids` 用于在本轮请求中排除已耗尽的 key，传 `None` 表示不排除。
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

    /// 立即刷新整个池（全量 discover）。`depleted_ids` 中的 key 在刷新后立即标记为耗尽，
    /// 防止它们在本轮请求中被重新选中。
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
                info!("池已刷新，{} 个 key 可用", count);
                true
            }
            Err(e) => {
                error!("池刷新失败: {e}");
                false
            }
        }
    }

    /// 记录一条代理请求日志。
    pub async fn record_log(&self, entry: LogEntry) {
        self.log_store.record(entry).await;
    }

    /// 将当前 sticky key 标记为耗尽，并异步刷新该工作区的用量数据。
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
                    .refresh_workspace_usage(&account_name, &ws_id)
                    .await
                {
                    warn!(
                        "后台用量刷新失败 workspace={}/{}: {e}",
                        account_name, ws_id
                    );
                } else {
                    info!("已刷新耗尽 key {} 的用量数据", PoolKey::mask_value(&key_value));
                }
            });
        }
    }

    /// 刷新单个工作区的 Go 用量（轻量操作，不走全量 discover）。
    /// 更新池中该工作区下所有 key 的 go_usage。
    async fn refresh_workspace_usage(
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
                .ok_or_else(|| anyhow::anyhow!("账户 '{account_name}' 不在配置中"))?
        };

        let oc = OpencodeClient::new(&account.name, &account.auth);
        let billing = oc.get_billing(workspace_id).await?;
        let go_usage = if billing.subscribed {
            oc.get_go_usage(workspace_id).await.unwrap_or_else(|e| {
                warn!("获取工作区 '{workspace_id}' 的 Go 用量失败: {e}");
                None
            })
        } else {
            None
        };

        let mut pool = self.inner.write().await;
        for k in &mut pool.keys {
            if k.workspace_id == workspace_id && k.account_name == account_name {
                k.go_usage = go_usage.clone();
                // 如果三个窗口都不满，清除耗尽标记
                if !k.is_fully_exhausted() {
                    k.depleted = false;
                    info!("key {} 耗尽标记已清除（用量已恢复）", k.masked_key());
                }
            }
        }

        info!("工作区 {workspace_id} 的 Go 用量已刷新");
        Ok(())
    }
}
