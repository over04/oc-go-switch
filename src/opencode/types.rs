use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workspace {
    pub id: String,
    pub name: String,
    pub slug: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKeyEntry {
    pub id: String,
    pub key: String,
    pub name: String,
    #[serde(rename = "createdAt", default)]
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BillingInfo {
    pub plan: Option<SubscriptionPlan>,
    pub subscribed: bool,
}

/// Go（lite）订阅用量数据，从工作区 Go 页面刮取。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoUsage {
    /// 小时滚动窗口用量百分比 (0–100)
    pub hourly_percent: u32,
    /// 距离小时窗口重置的秒数
    pub hourly_reset_sec: u64,
    /// 周滚动窗口用量百分比 (0–100)
    pub weekly_percent: u32,
    /// 距离周窗口重置的秒数
    pub weekly_reset_sec: u64,
    /// 月滚动窗口用量百分比 (0–100)
    pub monthly_percent: u32,
    /// 距离月窗口重置的秒数
    pub monthly_reset_sec: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SubscriptionPlan {
    Go,
    Zen,
}
