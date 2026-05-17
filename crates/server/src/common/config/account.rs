use serde::{Deserialize, Serialize};

/// OpenCode 账户配置。
///
/// 一个账户可发现多个工作区；调度时账户只作为工作区分组和鉴权来源。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountConfig {
    /// 账户唯一名称，用于组成内部工作区 id。
    pub name: String,
    /// OpenCode auth cookie 内容，服务端仅用于发现工作区和 key。
    pub auth: String,
    /// 前端展示标签，用于区分同一用户配置的多个账户。
    pub label: String,
}
