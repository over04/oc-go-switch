use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use arc_swap::ArcSwap;
use chrono::Utc;
use tokio::sync::{RwLock, RwLockReadGuard};
use tracing::{error, info};

use crate::{
    business::{
        log::store::LogStore,
        workspace::{
            client_set::ClientSet,
            discovery::discover,
            error::PoolError,
            scheduler::{KeyPool, SelectedKey},
        },
    },
    common::{
        config::{
            fixed::FixedConfig, runtime::RuntimeConfig, runtime_store::ConfigRuntime,
            store::ConfigStore, Config,
        },
        model::log::LogEntry,
    },
};

/// 工作区调度运行时句柄。
///
/// 请求路径通过该类型读取配置快照、选择 key、刷新工作区和写入代理日志。
/// 配置使用 `watch` 发布快照，避免请求路径共享写锁。
#[derive(Debug, Clone)]
pub struct KeyPoolHandle {
    inner: Arc<RwLock<KeyPool>>,
    fixed_config: Arc<FixedConfig>,
    config_runtime: ConfigRuntime,
    config_store: ConfigStore,
    clients: Arc<ArcSwap<ClientSet>>,
    log_store: Arc<LogStore>,
    refresh_running: Arc<AtomicBool>,
}

impl KeyPoolHandle {
    pub fn try_new(
        pool: KeyPool,
        config: Config,
        config_store: ConfigStore,
        log_store: Arc<LogStore>,
    ) -> Result<Self, PoolError> {
        let clients = ClientSet::try_new(&config.runtime.go)?;
        let fixed_config = Arc::new(config.fixed);
        let config_runtime = ConfigRuntime::new(config.runtime);

        Ok(Self {
            inner: Arc::new(RwLock::new(pool)),
            fixed_config,
            config_runtime,
            config_store,
            clients: Arc::new(ArcSwap::from_pointee(clients)),
            log_store,
            refresh_running: Arc::new(AtomicBool::new(false)),
        })
    }

    pub async fn read(&self) -> RwLockReadGuard<'_, KeyPool> {
        self.inner.read().await
    }

    pub fn fixed_config(&self) -> Arc<FixedConfig> {
        self.fixed_config.clone()
    }

    pub fn runtime_config(&self) -> Arc<RuntimeConfig> {
        self.config_runtime.current()
    }

    pub fn clients(&self) -> Arc<ClientSet> {
        self.clients.load_full()
    }

    pub async fn save_runtime_config(&self, next: RuntimeConfig) -> Result<(), PoolError> {
        next.validate()?;
        let clients = ClientSet::try_new(&next.go)?;
        let config = Config {
            fixed: self.fixed_config.as_ref().clone(),
            runtime: next.clone(),
        };
        self.config_store.save(&config).await?;
        self.clients.store(Arc::new(clients));
        self.config_runtime.replace(next);
        Ok(())
    }

    pub async fn wait_config_change(&self) {
        self.config_runtime.changed().await;
    }

    pub async fn set_active_key(&self, key_id: String) -> bool {
        self.inner.write().await.set_active_key(&key_id)
    }

    pub async fn clear_active_key(&self) {
        self.inner.write().await.clear_active_key();
    }

    pub async fn select_key_or_refresh(&self) -> Option<SelectedKey> {
        let selected = self.inner.write().await.select();
        if selected.is_none() {
            self.request_refresh();
        }
        selected
    }

    pub fn request_refresh(&self) {
        let handle = self.clone();
        tokio::spawn(async move {
            if let Err(error) = handle.refresh_now().await {
                error!("后台刷新失败: {error}");
            }
        });
    }

    pub async fn refresh_now(&self) -> Result<bool, PoolError> {
        let _guard = match RefreshGuard::try_acquire(self.refresh_running.clone()) {
            Some(guard) => guard,
            None => return Ok(false),
        };

        let config = self.runtime_config();
        let mut new_pool = discover(config.as_ref()).await?;
        new_pool.last_refresh_at = Some(Utc::now().to_rfc3339());
        let count = new_pool.available_workspace_count();

        *self.inner.write().await = new_pool;
        info!("池已刷新，{} 个 Go 工作区可用", count);
        Ok(true)
    }

    pub async fn record_log(&self, entry: LogEntry) {
        self.log_store.record(entry).await;
    }

    pub async fn list_logs(
        &self,
        limit: usize,
        direction: Option<crate::common::model::direction::Direction>,
        success: Option<bool>,
    ) -> Vec<LogEntry> {
        self.log_store.list(limit, direction, success).await
    }

    pub async fn clear_logs(&self) {
        self.log_store.clear().await;
    }

    pub async fn drop_workspace(&self, workspace_id: &str) {
        let empty = self.inner.write().await.drop_workspace(workspace_id);
        info!("workspace 已出队: {workspace_id}");
        if empty {
            self.request_refresh();
        }
    }
}

#[derive(Debug)]
struct RefreshGuard {
    running: Arc<AtomicBool>,
}

impl RefreshGuard {
    fn try_acquire(running: Arc<AtomicBool>) -> Option<Self> {
        running
            .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
            .is_ok()
            .then_some(Self { running })
    }
}

impl Drop for RefreshGuard {
    fn drop(&mut self) {
        self.running.store(false, Ordering::Release);
    }
}
