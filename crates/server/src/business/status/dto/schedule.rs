use serde::Serialize;

use crate::business::status::dto::account::AccountStatus;

#[derive(Debug, Serialize)]
pub struct WorkspaceScheduleResponse {
    /// 当前固定调度 key id。
    pub current_key_id: Option<String>,
    /// 最近一次完整刷新完成时间。
    pub last_refresh_at: Option<String>,
    /// 按账户分组的工作区调度状态。
    pub accounts: Vec<AccountStatus>,
}
