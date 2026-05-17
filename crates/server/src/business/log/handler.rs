use axum::{
    extract::{Query, State},
    response::Json,
};

use crate::{
    business::{log::query::LogListQuery, workspace::handle::KeyPoolHandle},
    common::model::log::LogEntry,
};

pub async fn list_logs(
    State(handle): State<KeyPoolHandle>,
    Query(query): Query<LogListQuery>,
) -> Json<Vec<LogEntry>> {
    let limit = query.limit.unwrap_or(100);
    Json(
        handle
            .list_logs(limit, query.direction, query.success)
            .await,
    )
}

pub async fn clear_logs(State(handle): State<KeyPoolHandle>) -> &'static str {
    handle.clear_logs().await;
    "ok"
}
