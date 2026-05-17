use std::collections::VecDeque;

use tokio::sync::Mutex;

use crate::common::model::{direction::Direction, log::LogEntry};

/// 内存请求日志。
///
/// 只保留最近 500 条代理请求记录，用于前端状态页展示和排障。
#[derive(Debug)]
pub struct LogStore {
    entries: Mutex<VecDeque<LogEntry>>,
}

impl LogStore {
    pub fn new() -> Self {
        Self {
            entries: Mutex::new(VecDeque::with_capacity(500)),
        }
    }

    pub async fn record(&self, entry: LogEntry) {
        let mut entries = self.entries.lock().await;
        if entries.len() >= 500 {
            entries.pop_front();
        }
        entries.push_back(entry);
    }

    pub async fn list(
        &self,
        limit: usize,
        direction: Option<Direction>,
        success: Option<bool>,
    ) -> Vec<LogEntry> {
        self.entries
            .lock()
            .await
            .iter()
            .rev()
            .filter(|entry| {
                direction
                    .as_ref()
                    .is_none_or(|direction| entry.direction == *direction)
                    && success.is_none_or(|success| entry.success == success)
            })
            .take(limit)
            .cloned()
            .collect()
    }

    pub async fn clear(&self) {
        self.entries.lock().await.clear();
    }
}
