use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoUsage {
    /// 小时滚动窗口用量百分比。
    pub hourly_percent: u32,
    /// 小时窗口重置剩余秒数。
    pub hourly_reset_sec: u64,
    /// 周滚动窗口用量百分比。
    pub weekly_percent: u32,
    /// 周窗口重置剩余秒数。
    pub weekly_reset_sec: u64,
    /// 月滚动窗口用量百分比。
    pub monthly_percent: u32,
    /// 月窗口重置剩余秒数。
    pub monthly_reset_sec: u64,
}

impl GoUsage {
    pub fn is_exhausted(&self) -> bool {
        self.hourly_percent >= 100 || self.weekly_percent >= 100 || self.monthly_percent >= 100
    }

    pub fn peak_percent(&self) -> u32 {
        self.hourly_percent
            .max(self.weekly_percent)
            .max(self.monthly_percent)
    }
}
