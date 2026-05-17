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
use config::store::ConfigStore;
use pool::pool::{discover, make_handle};
use tokio::sync::RwLock;
use tracing::{error, info};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_target(false)
        .with_thread_ids(false)
        .init();

    let config_store = ConfigStore::new("config.yaml");
    let config = config_store.load().await.unwrap_or_else(|e| {
        eprintln!("加载 config.yaml 失败: {e}");
        std::process::exit(1);
    });

    let config = Arc::new(RwLock::new(config));

    info!(
        "启动 oc-go-switch，已配置 {} 个账户",
        config.read().await.accounts.len()
    );

    let pool = {
        let config_guard = config.read().await;
        discover(&config_guard).await.unwrap_or_else(|e| {
            error!("发现 key 失败: {e}");
            std::process::exit(1);
        })
    };

    let total_keys: usize = pool
        .workspaces
        .values()
        .map(|workspace| workspace.keys.len())
        .sum();
    let total_workspaces = pool.workspaces.len();
    info!("KeyPool 就绪: {total_workspaces} 个 Go 工作区，{total_keys} 个 key");

    let log_store = Arc::new(LogStore::new());
    let handle = make_handle(pool, config.clone(), config_store, log_store);

    // 后台定时刷新任务
    let refresh_interval = config.read().await.refresh_interval_secs;
    if refresh_interval > 0 {
        let refresh_handle = handle.clone();
        tokio::spawn(async move {
            loop {
                tokio::time::sleep(std::time::Duration::from_secs(refresh_interval)).await;
                info!("后台刷新: 更新余额...");
                match refresh_handle.refresh_now().await {
                    Ok(true) => info!("后台刷新完成"),
                    Ok(false) => info!("已有刷新任务运行，跳过本轮后台刷新"),
                    Err(e) => error!("后台刷新失败: {e}"),
                }
            }
        });
    }

    let listen = config.read().await.listen.clone();
    let addr: SocketAddr = listen
        .parse()
        .unwrap_or_else(|e: std::net::AddrParseError| {
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
