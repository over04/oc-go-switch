use serde::Serialize;

use crate::business::status::dto::workspace::WorkspaceStatus;

#[derive(Debug, Serialize)]
pub struct DashboardStatusResponse {
    /// 已发现的 key 总数，包含不可调度工作区里的 key。
    pub total_keys: usize,
    /// 可调度工作区里的 key 数量。
    pub available_keys: usize,
    /// 可调度工作区数量。
    pub available_workspaces: usize,
    /// 当前无额度的工作区数量。
    pub exhausted_workspaces: usize,
    /// 缺少 Go 订阅的工作区数量。
    pub unsubscribed_workspaces: usize,
    /// 最近一次完整刷新完成时间。
    pub last_refresh_at: Option<String>,
    /// 仪表盘展示用工作区用量。
    pub go_workspaces: Vec<WorkspaceStatus>,
}
