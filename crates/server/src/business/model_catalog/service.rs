use crate::business::{
    model_catalog::model_list::ModelListResponse, workspace::handle::KeyPoolHandle,
};

pub async fn fetch_model_list(
    handle: &KeyPoolHandle,
    base_url: &str,
) -> Result<ModelListResponse, String> {
    let key = handle
        .select_key_or_refresh()
        .await
        .ok_or_else(|| "没有可用 Go 工作区".to_string())?;
    let url = format!("{}/models", base_url);

    let resp = handle
        .short_client
        .get(&url)
        .header("Authorization", format!("Bearer {}", key.key_value))
        .send()
        .await
        .map_err(|error| format!("上游不可达: {error}"))?;

    let status = resp.status();
    let body = resp
        .text()
        .await
        .map_err(|error| format!("读取错误: {error}"))?;

    if !status.is_success() {
        return Err(format!("HTTP {}: {}", status.as_u16(), body));
    }

    serde_json::from_str(&body).map_err(|error| format!("JSON 解析错误: {error}"))
}
