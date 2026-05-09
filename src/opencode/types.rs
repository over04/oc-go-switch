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
    pub balance: i64,
    pub plan: Option<SubscriptionPlan>,
    pub subscribed: bool,
}

/// Go (lite) subscription usage data scraped from the workspace Go page.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoUsage {
    /// Hourly rolling window usage percent (0-100)
    pub hourly_percent: u32,
    /// Seconds until hourly reset
    pub hourly_reset_sec: u64,
    /// Weekly usage percent (0-100)
    pub weekly_percent: u32,
    /// Seconds until weekly reset
    pub weekly_reset_sec: u64,
    /// Monthly usage percent (0-100)
    pub monthly_percent: u32,
    /// Seconds until monthly reset
    pub monthly_reset_sec: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SubscriptionPlan {
    Go,
    Zen,
}
