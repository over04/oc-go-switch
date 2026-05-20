use std::{net::SocketAddr, sync::Arc, time::Duration};

use tracing::{error, info};

use crate::{
    business::{
        log::store::LogStore,
        workspace::{discovery::discover, handle::KeyPoolHandle, scheduler::KeyPool},
    },
    common::config::{store::ConfigStore, Config},
    init::router::build_router,
};

pub async fn run() {
    let config_store = ConfigStore::new("config.yaml");
    let config = load_config(&config_store).await;

    info!(
        "启动 oc-go-switch，已配置 {} 个账户",
        config.runtime.accounts.len()
    );

    let pool = discover(&config.runtime).await.unwrap_or_else(|error| {
        error!("发现 key 失败: {error}");
        std::process::exit(1);
    });
    log_pool_ready(&pool);

    let listen = config.fixed.listen.clone();
    let handle = KeyPoolHandle::try_new(pool, config, config_store, Arc::new(LogStore::new()))
        .unwrap_or_else(|error| {
            error!("初始化 key pool 失败: {error}");
            std::process::exit(1);
        });

    spawn_refresh_task(&handle);
    serve(listen, handle).await;
}

async fn load_config(config_store: &ConfigStore) -> Config {
    config_store.load().await.unwrap_or_else(|error| {
        eprintln!("加载 config.yaml 失败: {error}");
        std::process::exit(1);
    })
}

fn log_pool_ready(pool: &KeyPool) {
    let total_keys: usize = pool
        .workspaces
        .values()
        .map(|workspace| workspace.keys.len())
        .sum();
    let total_workspaces = pool.workspaces.len();
    info!("KeyPool 就绪: {total_workspaces} 个 Go 工作区，{total_keys} 个 key");
}

fn spawn_refresh_task(handle: &KeyPoolHandle) {
    let refresh_handle = handle.clone();
    tokio::spawn(async move {
        loop {
            let refresh_interval = refresh_handle.runtime_config().refresh_interval_secs;
            if refresh_interval == 0 {
                refresh_handle.wait_config_change().await;
                continue;
            }
            tokio::select! {
                _ = tokio::time::sleep(Duration::from_secs(refresh_interval)) => {}
                _ = refresh_handle.wait_config_change() => {
                    continue;
                }
            }
            info!("后台刷新: 更新余额...");
            match refresh_handle.refresh_now().await {
                Ok(true) => info!("后台刷新完成"),
                Ok(false) => info!("已有刷新任务运行，跳过本轮后台刷新"),
                Err(error) => error!("后台刷新失败: {error}"),
            }
        }
    });
}

async fn serve(listen: String, handle: KeyPoolHandle) {
    let addr: SocketAddr = listen.parse().unwrap_or_else(|error| {
        error!("无效的监听地址 '{listen}': {error}");
        std::process::exit(1);
    });

    let router = build_router(handle);
    info!("代理正在监听 http://{addr}");

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .unwrap_or_else(|error| {
            error!("绑定 {addr} 失败: {error}");
            std::process::exit(1);
        });

    axum::serve(listener, router).await.unwrap_or_else(|error| {
        error!("服务器错误: {error}");
        std::process::exit(1);
    });
}
