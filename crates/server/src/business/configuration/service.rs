use axum::http::StatusCode;

pub fn validate_token_input(value: String) -> Result<String, StatusCode> {
    if value.trim().is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }
    Ok(value)
}
