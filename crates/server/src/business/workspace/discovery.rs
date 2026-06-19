use std::collections::HashMap;

use tracing::{error, info, warn};

use crate::{
    business::workspace::{
        credential::WorkspaceCredential, error::PoolError, record::WorkspacePool,
        scheduler::WorkspaceScheduler, status::WorkspacePoolStatus,
    },
    common::config::runtime::RuntimeConfig,
};
use adapter::opencode::{client::OpencodeClient, model::subscription_plan::SubscriptionPlan};

/// 从 OpenCode 账户发现工作区、订阅状态、Go 用量和工作区内 key。
///
/// 发现结果保留 available、exhausted、unsubscribed 三类工作区；
/// 调度队列只从 available 工作区构建。
pub async fn discover(config: &RuntimeConfig) -> Result<WorkspaceScheduler, PoolError> {
    let mut workspaces: HashMap<String, WorkspacePool> = HashMap::new();

    for account in &config.accounts {
        info!(
            "正在发现账户 '{}' ({}) 的工作区...",
            account.name, account.label
        );

        let oc = OpencodeClient::new(&account.name, &account.auth).map_err(PoolError::Discover)?;
        let remote_workspaces = match oc.get_workspaces().await {
            Ok(workspaces) => workspaces,
            Err(error) => {
                error!("获取账户 '{}' 的工作区列表失败: {error}", account.name);
                continue;
            }
        };

        for workspace in &remote_workspaces {
            let billing = match oc.get_billing(&workspace.id).await {
                Ok(billing) => billing,
                Err(error) => {
                    warn!("获取工作区 '{}' 的账单信息失败: {error}", workspace.name);
                    continue;
                }
            };

            let key_entries = match oc.list_keys(&workspace.id).await {
                Ok(keys) => keys,
                Err(error) => {
                    warn!(
                        "获取工作区 '{}' ({}/{}) 的 key 列表失败: {error}",
                        workspace.name, account.name, workspace.id
                    );
                    continue;
                }
            };

            let is_go = billing.subscribed && billing.plan == Some(SubscriptionPlan::Go);
            let go_usage = if is_go {
                match oc.get_go_usage(&workspace.id).await {
                    Ok(usage) => usage,
                    Err(error) => {
                        warn!("获取工作区 '{}' 的 Go 用量失败: {error}", workspace.name);
                        None
                    }
                }
            } else {
                None
            };

            let Some(key_entry) = key_entries.into_iter().next() else {
                continue;
            };
            let credential = WorkspaceCredential {
                value: key_entry.key,
            };

            let workspace_id = format!("{}/{}", account.name, workspace.id);
            let status = match (
                is_go,
                go_usage.as_ref().is_some_and(|usage| usage.is_exhausted()),
            ) {
                (false, _) => WorkspacePoolStatus::Unsubscribed,
                (true, true) => WorkspacePoolStatus::Exhausted,
                (true, false) => WorkspacePoolStatus::Available,
            };

            workspaces.insert(
                workspace_id.clone(),
                WorkspacePool {
                    id: workspace_id,
                    name: workspace.name.clone(),
                    account_name: account.name.clone(),
                    account_label: account.label.clone(),
                    status,
                    plan: billing.plan,
                    go_usage,
                    credential,
                },
            );
        }
    }

    let pool = WorkspaceScheduler::new(workspaces);
    if pool.workspaces.is_empty() {
        return Err(PoolError::NoWorkspace);
    }

    info!(
        "WorkspaceScheduler: 共发现 {} 个工作区，{} 个可调度 Go 工作区",
        pool.workspaces.len(),
        pool.available_workspace_count()
    );
    Ok(pool)
}
