use adapter::opencode::model::go_usage::GoUsage;
use serde::Serialize;

use crate::business::status::dto::{
    key::KeyStatusEntry, workspace_queue_status::WorkspaceQueueStatus,
};

#[derive(Debug, Serialize)]
pub struct WorkspaceStatus {
    /// 调度内部工作区 id。
    pub id: String,
    /// OpenCode 工作区展示名。
    pub name: String,
    /// 工作区三态：可用、当前无额度、无订阅。
    pub status: WorkspaceQueueStatus,
    /// 当前最近一次请求是否使用该工作区。
    pub is_current: bool,
    /// 在调度队列中的位置；无值表示当前不参与调度。
    pub queue_position: Option<usize>,
    /// OpenCode 订阅计划展示值。
    pub plan: Option<String>,
    /// Go 用量数据。
    pub go_usage: Option<GoUsage>,
    /// 工作区内 key 展示状态。
    pub keys: Vec<KeyStatusEntry>,
}
