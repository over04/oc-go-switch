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
    pub balance_cents: i64,
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
