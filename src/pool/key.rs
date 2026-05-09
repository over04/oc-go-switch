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

    /// 三个滚动窗口中任一达到 100% 即为完全耗尽。上游会直接拒绝该 key 的请求。
    pub fn is_fully_exhausted(&self) -> bool {
        self.go_usage.as_ref().is_some_and(|u| {
            u.hourly_percent >= 100 || u.weekly_percent >= 100 || u.monthly_percent >= 100
        })
    }

    /// 三个窗口中用量最高的百分比。没有 go_usage 数据时返回 0（乐观）。
    pub fn max_usage_pct(&self) -> u32 {
        self.go_usage.as_ref().map_or(0, |u| {
            u.hourly_percent.max(u.weekly_percent).max(u.monthly_percent)
        })
    }

    /// 脱敏显示 key：显示前 6 位和后 4 位。
    pub fn masked_key(&self) -> String {
        Self::mask_value(&self.key_value)
    }

    /// 静态工具方法：对任意 key 字符串脱敏。
    pub fn mask_value(raw: &str) -> String {
        if raw.len() <= 12 {
            return "***".to_string();
        }
        let prefix = &raw[..6];
        let suffix = &raw[raw.len() - 4..];
        format!("{prefix}...{suffix}")
    }
}
