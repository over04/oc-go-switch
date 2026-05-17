use std::collections::{HashMap, VecDeque};
use std::sync::Arc;

use chrono::Utc;
use tokio::sync::{Mutex, RwLock};
use tracing::{error, info, warn};

use crate::api::logs::LogStore;
use crate::config::{store::ConfigStore, Config};
use crate::model::LogEntry;
use crate::opencode::client::OpencodeClient;
use crate::opencode::types::{GoUsage, SubscriptionPlan};

use super::key::PoolKey;

#[derive(Debug, Clone)]
pub struct WorkspacePool {
    pub id: String,
    pub name: String,
    pub account_name: String,
    pub account_label: String,
    pub status: WorkspacePoolStatus,
    pub plan: Option<SubscriptionPlan>,
    pub go_usage: Option<GoUsage>,
    pub keys: VecDeque<PoolKey>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkspacePoolStatus {
    Available,
    Exhausted,
    Unsubscribed,
}

impl WorkspacePool {
    fn usage_rank(&self) -> u32 {
        self.go_usage.as_ref().map_or(u32::MAX, |u| {
            u.hourly_percent
                .max(u.weekly_percent)
                .max(u.monthly_percent)
        })
    }
}

#[derive(Debug)]
pub struct KeyPool {
    pub workspaces: HashMap<String, WorkspacePool>,
    pub workspace_queue: VecDeque<String>,
    pub current_workspace_id: Option<String>,
    pub current_key_id: Option<String>,
    pub last_refresh_at: Option<String>,
}

#[derive(Debug, Clone)]
pub struct SelectedKey {
    pub id: String,
    pub key_value: String,
    pub workspace_id: String,
    pub workspace_name: String,
}

impl SelectedKey {
    pub fn masked_key(&self) -> String {
        PoolKey::mask_value(&self.key_value)
    }
}

#[derive(Debug, Clone)]
pub struct KeyPoolHandle {
    pub(crate) inner: Arc<RwLock<KeyPool>>,
    pub(crate) config: Arc<RwLock<Config>>,
    pub(crate) config_store: ConfigStore,
    pub log_store: Arc<LogStore>,
    pub proxy_client: reqwest::Client,
    pub short_client: reqwest::Client,
    refresh_gate: Arc<Mutex<()>>,
}

impl KeyPoolHandle {
    pub async fn read(&self) -> tokio::sync::RwLockReadGuard<'_, KeyPool> {
        self.inner.read().await
    }

    pub async fn config_snapshot(&self) -> Config {
        self.config.read().await.clone()
    }

    pub async fn save_config_snapshot(&self, next: Config) -> Result<(), anyhow::Error> {
        next.validate()?;
        self.config_store.save(&next).await?;
        let mut config = self.config.write().await;
        *config = next;
        Ok(())
    }

    pub async fn set_active_key(&self, key_id: String) -> bool {
        let mut pool = self.inner.write().await;
        let Some((workspace_id, key)) = take_key_from_any_workspace(&mut pool, &key_id) else {
            return false;
        };
        pool.current_workspace_id = Some(workspace_id.clone());
        pool.current_key_id = Some(key.id.clone());
        if let Some(workspace) = pool.workspaces.get_mut(&workspace_id) {
            workspace.keys.push_front(key);
        }
        move_workspace_front(&mut pool.workspace_queue, &workspace_id);
        true
    }

    pub async fn clear_active_key(&self) {
        let mut pool = self.inner.write().await;
        pool.current_workspace_id = None;
        pool.current_key_id = None;
    }
}

pub async fn discover(config: &Config) -> anyhow::Result<KeyPool> {
    let mut workspaces: HashMap<String, WorkspacePool> = HashMap::new();

    for account in &config.accounts {
        info!(
            "正在发现账户 '{}' ({}) 的工作区...",
            account.name, account.label
        );

        let oc = OpencodeClient::new(&account.name, &account.auth)
            .map_err(|e| anyhow::anyhow!("{e}"))?;
        let remote_workspaces = match oc.get_workspaces().await {
            Ok(ws) => ws,
            Err(e) => {
                error!("获取账户 '{}' 的工作区列表失败: {e}", account.name);
                continue;
            }
        };

        for ws in &remote_workspaces {
            let billing = match oc.get_billing(&ws.id).await {
                Ok(b) => b,
                Err(e) => {
                    warn!("获取工作区 '{}' 的账单信息失败: {e}", ws.name);
                    continue;
                }
            };

            let key_entries = match oc.list_keys(&ws.id).await {
                Ok(keys) => keys,
                Err(e) => {
                    warn!(
                        "获取工作区 '{}' ({}/{}) 的 key 列表失败: {e}",
                        ws.name, account.name, ws.id
                    );
                    continue;
                }
            };

            let is_go = billing.subscribed && billing.plan == Some(SubscriptionPlan::Go);
            let go_usage = if is_go {
                match oc.get_go_usage(&ws.id).await {
                    Ok(usage) => usage,
                    Err(e) => {
                        warn!("获取工作区 '{}' 的 Go 用量失败: {e}", ws.name);
                        None
                    }
                }
            } else {
                None
            };

            let mut key_queue = VecDeque::new();
            for key_entry in key_entries {
                key_queue.push_back(PoolKey {
                    id: format!("{}/{}/{}", account.name, ws.id, key_entry.id),
                    key_value: key_entry.key,
                    key_name: key_entry.name,
                });
            }

            if key_queue.is_empty() {
                continue;
            }

            let workspace_id = format!("{}/{}", account.name, ws.id);
            let status = if !is_go {
                WorkspacePoolStatus::Unsubscribed
            } else if is_exhausted_usage(go_usage.as_ref()) {
                WorkspacePoolStatus::Exhausted
            } else {
                WorkspacePoolStatus::Available
            };
            workspaces.insert(
                workspace_id.clone(),
                WorkspacePool {
                    id: workspace_id,
                    name: ws.name.clone(),
                    account_name: account.name.clone(),
                    account_label: account.label.clone(),
                    status,
                    plan: billing.plan,
                    go_usage,
                    keys: key_queue,
                },
            );
        }
    }

    if !workspaces
        .values()
        .any(|workspace| workspace.status == WorkspacePoolStatus::Available)
    {
        return Err(anyhow::anyhow!(
            "未发现可用的 Go 工作区，请检查配置和账户授权。"
        ));
    }

    let mut workspace_queue: VecDeque<String> = workspaces
        .iter()
        .filter(|(_, workspace)| workspace.status == WorkspacePoolStatus::Available)
        .map(|(id, _)| id.clone())
        .collect();
    workspace_queue.make_contiguous().sort_by_key(|id| {
        workspaces
            .get(id)
            .map_or(u32::MAX, WorkspacePool::usage_rank)
    });

    info!(
        "KeyPool: 共发现 {} 个工作区，{} 个可调度 Go 工作区",
        workspaces.len(),
        workspace_queue.len()
    );

    Ok(KeyPool {
        workspaces,
        workspace_queue,
        current_workspace_id: None,
        current_key_id: None,
        last_refresh_at: None,
    })
}

pub fn make_handle(
    pool: KeyPool,
    config: Arc<RwLock<Config>>,
    config_store: ConfigStore,
    log_store: Arc<LogStore>,
) -> KeyPoolHandle {
    let proxy_client = reqwest::Client::builder()
        .connect_timeout(std::time::Duration::from_secs(10))
        .build()
        .expect("构建代理 HTTP client 失败");
    let short_client = reqwest::Client::builder()
        .connect_timeout(std::time::Duration::from_secs(10))
        .timeout(std::time::Duration::from_secs(60))
        .build()
        .expect("构建短请求 HTTP client 失败");

    KeyPoolHandle {
        inner: Arc::new(RwLock::new(pool)),
        config,
        config_store,
        log_store,
        proxy_client,
        short_client,
        refresh_gate: Arc::new(Mutex::new(())),
    }
}

impl KeyPoolHandle {
    pub async fn select_key_or_refresh(&self) -> Option<SelectedKey> {
        let selected = {
            let mut pool = self.inner.write().await;
            select_from_pool(&mut pool)
        };

        if selected.is_none() {
            self.request_refresh();
        }
        selected
    }

    pub fn request_refresh(&self) {
        let handle = self.clone();
        tokio::spawn(async move {
            if let Err(e) = handle.refresh_now().await {
                error!("后台刷新失败: {e}");
            }
        });
    }

    pub async fn refresh_now(&self) -> Result<bool, anyhow::Error> {
        let _refresh_guard = match self.refresh_gate.try_lock() {
            Ok(guard) => guard,
            Err(_) => return Ok(false),
        };

        let config = self.config_snapshot().await;
        let mut new_pool = discover(&config).await?;
        new_pool.last_refresh_at = Some(Utc::now().to_rfc3339());
        let count = new_pool.workspace_queue.len();

        let mut pool = self.inner.write().await;
        *pool = new_pool;
        info!("池已刷新，{} 个 Go 工作区可用", count);
        Ok(true)
    }

    pub async fn record_log(&self, entry: LogEntry) {
        self.log_store.record(entry).await;
    }

    pub async fn drop_workspace(&self, workspace_id: &str) {
        let empty = {
            let mut pool = self.inner.write().await;
            pool.workspaces.remove(workspace_id);
            pool.workspace_queue.retain(|id| id != workspace_id);
            if pool.current_workspace_id.as_deref() == Some(workspace_id) {
                pool.current_workspace_id = None;
                pool.current_key_id = None;
            }
            pool.workspace_queue.is_empty()
        };
        info!("workspace 已出队: {workspace_id}");
        if empty {
            self.request_refresh();
        }
    }
}

fn select_from_pool(pool: &mut KeyPool) -> Option<SelectedKey> {
    let workspace_id = pool.workspace_queue.pop_front()?;
    let key = {
        let workspace = pool.workspaces.get_mut(&workspace_id)?;
        workspace.keys.pop_front()?
    };

    let selected = {
        let workspace = pool.workspaces.get(&workspace_id)?;
        SelectedKey {
            id: key.id.clone(),
            key_value: key.key_value.clone(),
            workspace_id: workspace.id.clone(),
            workspace_name: workspace.name.clone(),
        }
    };

    let workspace = pool.workspaces.get_mut(&workspace_id)?;
    workspace.keys.push_back(key);
    pool.workspace_queue.push_back(workspace_id.clone());
    pool.current_workspace_id = Some(workspace_id);
    pool.current_key_id = Some(selected.id.clone());
    Some(selected)
}

fn take_key_from_any_workspace(pool: &mut KeyPool, key_id: &str) -> Option<(String, PoolKey)> {
    for (workspace_id, workspace) in &mut pool.workspaces {
        if let Some(index) = workspace.keys.iter().position(|key| key.id == key_id) {
            let key = workspace.keys.remove(index)?;
            return Some((workspace_id.clone(), key));
        }
    }
    None
}

fn move_workspace_front(queue: &mut VecDeque<String>, workspace_id: &str) {
    queue.retain(|id| id != workspace_id);
    queue.push_front(workspace_id.to_string());
}

fn is_exhausted_usage(go_usage: Option<&GoUsage>) -> bool {
    go_usage.is_some_and(|u| {
        u.hourly_percent >= 100 || u.weekly_percent >= 100 || u.monthly_percent >= 100
    })
}
