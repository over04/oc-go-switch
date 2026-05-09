use serde::Serialize;

use crate::opencode::types::{GoUsage, SubscriptionPlan};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum KeyStatus {
    Active,
    Idle,
    Depleted,
}

#[derive(Debug, Clone, Serialize)]
pub struct PoolKey {
    pub id: String,
    pub account_name: String,
    pub account_label: String,
    pub workspace_id: String,
    pub workspace_name: String,
    pub key_value: String,
    pub key_name: String,
    pub plan: Option<SubscriptionPlan>,
    pub subscribed: bool,
    pub depleted: bool,
    pub go_usage: Option<GoUsage>,
}

impl PoolKey {
    pub fn status(&self) -> KeyStatus {
        if self.depleted {
            KeyStatus::Depleted
        } else {
            KeyStatus::Idle
        }
    }

    /// True when ANY of the three rolling windows is at 100%.
    /// One window at 100% is enough — upstream will reject requests.
    pub fn is_fully_exhausted(&self) -> bool {
        self.go_usage.as_ref().is_some_and(|u| {
            u.hourly_percent >= 100 || u.weekly_percent >= 100 || u.monthly_percent >= 100
        })
    }

    /// Worst-case usage percentage across the three windows.
    /// Keys without go_usage data return 0 (optimistic).
    pub fn max_usage_pct(&self) -> u32 {
        self.go_usage.as_ref().map_or(0, |u| {
            u.hourly_percent.max(u.weekly_percent).max(u.monthly_percent)
        })
    }

    /// Masked key value for display: show first 6 and last 4 chars.
    pub fn masked_key(&self) -> String {
        Self::mask_value(&self.key_value)
    }

    /// Static helper: mask an arbitrary key string.
    pub fn mask_value(raw: &str) -> String {
        if raw.len() <= 12 {
            return "***".to_string();
        }
        let prefix = &raw[..6];
        let suffix = &raw[raw.len() - 4..];
        format!("{prefix}...{suffix}")
    }
}
