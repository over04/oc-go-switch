use adapter::opencode::model::{go_usage::GoUsage, subscription_plan::SubscriptionPlan};

use crate::business::workspace::{credential::WorkspaceCredential, status::WorkspacePoolStatus};

#[derive(Debug, Clone)]
pub struct WorkspacePool {
    /// 调度内部工作区 id，格式为 `account_name/workspace_id`。
    pub id: String,
    /// OpenCode 工作区展示名。
    pub name: String,
    /// 所属账户名称。
    pub account_name: String,
    /// 所属账户展示标签。
    pub account_label: String,
    /// 工作区当前调度状态。
    pub status: WorkspacePoolStatus,
    /// OpenCode 订阅计划。
    pub plan: Option<SubscriptionPlan>,
    /// Go 用量，只有 Go 订阅工作区才会存在。
    pub go_usage: Option<GoUsage>,
    /// 工作区代理凭证；同一工作区内 OpenCode key 共享额度，调度只保存一个凭证。
    pub credential: WorkspaceCredential,
}

impl WorkspacePool {
    pub fn usage_rank(&self) -> u32 {
        self.go_usage
            .as_ref()
            .map_or(u32::MAX, GoUsage::peak_percent)
    }
}
