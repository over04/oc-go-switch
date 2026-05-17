use axum::http::StatusCode;

pub fn is_quota_exhausted(status: StatusCode, body: &str) -> bool {
    (status == StatusCode::PAYMENT_REQUIRED || status == StatusCode::TOO_MANY_REQUESTS)
        && (body.contains("insufficient_quota")
            || body.contains("insufficient")
            || body.contains("balance"))
}
