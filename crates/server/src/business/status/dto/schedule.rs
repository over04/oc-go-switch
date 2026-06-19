use serde::Serialize;

use crate::business::status::dto::account::AccountStatus;

#[derive(Debug, Serialize)]
pub struct WorkspaceScheduleResponse {
    /// 当前亲和调度工作区 id；为空时使用普通工作区轮询。
    pub affinity_workspace_id: Option<String>,
    /// 最近一次完整刷新完成时间。
    pub last_refresh_at: Option<String>,
    /// 按账户分组的工作区调度状态。
    pub accounts: Vec<AccountStatus>,
}
