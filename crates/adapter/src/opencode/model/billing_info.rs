use serde::{Deserialize, Serialize};

use crate::opencode::model::subscription_plan::SubscriptionPlan;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BillingInfo {
    pub plan: Option<SubscriptionPlan>,
    pub subscribed: bool,
}
