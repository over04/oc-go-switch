use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::Duration,
};

use chrono::Utc;
use tokio::sync::{watch, RwLock, RwLockReadGuard};
use tracing::{error, info};

use crate::{
    business::{
        log::store::LogStore,
        workspace::{
            discovery::discover,
            error::PoolError,
            scheduler::{KeyPool, SelectedKey},
        },
    },
    common::{
        config::{store::ConfigStore, Config},
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
    config_tx: watch::Sender<Arc<Config>>,
    config_store: ConfigStore,
    log_store: Arc<LogStore>,
    pub proxy_client: reqwest::Client,
    pub short_client: reqwest::Client,
    refresh_running: Arc<AtomicBool>,
}

impl KeyPoolHandle {
    pub fn try_new(
        pool: KeyPool,
        config: Config,
        config_store: ConfigStore,
        log_store: Arc<LogStore>,
    ) -> Result<Self, PoolError> {
        let connect_timeout = Duration::from_secs(config.go.connect_timeout_secs);
        let request_timeout = Duration::from_secs(config.go.request_timeout_secs);
        let proxy_client = reqwest::Client::builder()
            .connect_timeout(connect_timeout)
            .build()?;
        let short_client = reqwest::Client::builder()
            .connect_timeout(connect_timeout)
            .timeout(request_timeout)
            .build()?;
        let (config_tx, _config_rx) = watch::channel(Arc::new(config));

        Ok(Self {
            inner: Arc::new(RwLock::new(pool)),
            config_tx,
            config_store,
            log_store,
            proxy_client,
            short_client,
            refresh_running: Arc::new(AtomicBool::new(false)),
        })
    }

    pub async fn read(&self) -> RwLockReadGuard<'_, KeyPool> {
        self.inner.read().await
    }

    pub fn config_snapshot(&self) -> Arc<Config> {
        self.config_tx.borrow().clone()
    }

    pub async fn save_config_snapshot(&self, next: Config) -> Result<(), PoolError> {
        next.validate()?;
        self.config_store.save(&next).await?;
        let _ = self.config_tx.send(Arc::new(next));
        Ok(())
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

        let config = self.config_snapshot();
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
