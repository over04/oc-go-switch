use axum::http::StatusCode;

use crate::business::workspace::handle::WorkspaceSchedulerHandle;

pub async fn refresh_workspace_pool(
    handle: &WorkspaceSchedulerHandle,
) -> Result<&'static str, StatusCode> {
    match handle.refresh_now().await {
        Ok(true) => Ok("ok"),
        Ok(false) => Ok("refresh already running"),
        Err(error) => {
            tracing::error!("强制刷新失败: {error}");
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
