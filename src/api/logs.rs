use axum::{extract::State, response::Json};
use serde::Deserialize;
use std::collections::VecDeque;
use tokio::sync::Mutex;

use crate::model::{Direction, LogEntry};
use crate::pool::pool::KeyPoolHandle;

#[derive(Debug)]
pub struct LogStore {
    pub entries: Mutex<VecDeque<LogEntry>>,
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
}

#[derive(Debug, Deserialize)]
pub struct LogsQuery {
    pub limit: Option<usize>,
    pub direction: Option<Direction>,
    pub success: Option<bool>,
}

/// GET /api/logs — 最近的代理请求日志
pub async fn list_logs(
    State(handle): State<KeyPoolHandle>,
    axum::extract::Query(query): axum::extract::Query<LogsQuery>,
) -> Json<Vec<LogEntry>> {
    let entries = handle.log_store.entries.lock().await;
    let entries: &VecDeque<LogEntry> = &entries;
    let limit = query.limit.unwrap_or(100);

    let filtered: Vec<LogEntry> = entries
        .iter()
        .rev()
        .filter(|e| {
            if let Some(ref dir) = query.direction {
                if &e.direction != dir {
                    return false;
                }
            }
            if let Some(success) = query.success {
                if e.success != success {
                    return false;
                }
            }
            true
        })
        .take(limit)
        .cloned()
        .collect();

    Json(filtered)
}

/// DELETE /api/logs — 清空所有日志
pub async fn clear_logs(State(handle): State<KeyPoolHandle>) -> &'static str {
    let mut entries = handle.log_store.entries.lock().await;
    entries.clear();
    "ok"
}
