#![allow(clippy::module_inception)]
mod api;
mod config;
mod model;
mod opencode;
mod pool;
mod protocol;
mod proxy;

use std::net::SocketAddr;
use std::sync::Arc;

use api::logs::LogStore;
use api::router::build_router;
use chrono::Utc;
use pool::pool::{discover, make_handle};
use tokio::sync::RwLock;
use tracing::{error, info};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_target(false)
        .with_thread_ids(false)
        .init();

    let config = config::Config::load("config.yaml").unwrap_or_else(|e| {
        eprintln!("加载 config.yaml 失败: {e}");
        std::process::exit(1);
    });

    let config = Arc::new(RwLock::new(config));

    info!(
        "启动 oc-go-switch，已配置 {} 个账户",
        config.read().await.accounts.len()
    );

    let config_guard = config.read().await;
    let pool = match discover(&config_guard).await {
        Ok(p) => p,
        Err(e) => {
            error!("发现 key 失败: {e}");
            std::process::exit(1);
        }
    };

    let total = pool.keys.len();
    let subscribed = pool.keys.iter().filter(|k| k.subscribed).count();
    info!("KeyPool 就绪: {total} 个 key（{subscribed} 个已订阅 Go）");

    let log_store = Arc::new(LogStore::new());
    let handle = make_handle(pool, config.clone(), log_store);

    // 后台定时刷新任务
    let refresh_interval = config.read().await.refresh_interval_secs;
    if refresh_interval > 0 {
        let refresh_handle = handle.clone();
        let refresh_config = config.clone();
        tokio::spawn(async move {
            loop {
                tokio::time::sleep(std::time::Duration::from_secs(refresh_interval)).await;
                info!("后台刷新: 更新余额...");
                let refresh_guard = refresh_config.read().await;
                match discover(&refresh_guard).await {
                    Ok(mut new_pool) => {
                        drop(refresh_guard);
                        let mut pool = refresh_handle.inner.write().await;
                        // 只有用量仍完全耗尽才保留 depleted 标记
                        let old_depleted_and_broke: std::collections::HashSet<String> = pool
                            .keys
                            .iter()
                            .filter(|k| k.depleted)
                            .map(|k| k.id.clone())
                            .collect();
                        for k in &mut new_pool.keys {
                            if old_depleted_and_broke.contains(&k.id) && k.is_fully_exhausted()
                            {
                                k.depleted = true;
                            }
                        }
                        new_pool.last_refresh_at = Some(Utc::now().to_rfc3339());
                        *pool = new_pool;
                        info!("后台刷新完成");
                    }
                    Err(e) => {
                        error!("后台刷新失败: {e}");
                    }
                }
            }
        });
    }

    let listen = config.read().await.listen.clone();
    let addr: SocketAddr = listen.parse().unwrap_or_else(|e: std::net::AddrParseError| {
        error!("无效的监听地址 '{listen}': {e}");
        std::process::exit(1);
    });

    let router = build_router(handle);
    info!("代理正在监听 http://{addr}");

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .unwrap_or_else(|e| {
            error!("绑定 {addr} 失败: {e}");
            std::process::exit(1);
        });

    axum::serve(listener, router)
        .await
        .unwrap_or_else(|e: std::io::Error| {
            error!("服务器错误: {e}");
            std::process::exit(1);
        });
}
