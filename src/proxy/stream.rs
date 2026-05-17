use axum::{http::StatusCode, response::Response};
use futures_util::StreamExt;

fn build_response(
    status: StatusCode,
    headers: &[(&str, &str)],
    body: axum::body::Body,
) -> Response {
    let mut builder = Response::builder().status(status);
    for (k, v) in headers {
        builder = builder.header(*k, *v);
    }
    builder.body(body).unwrap_or_else(|_| {
        Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(axum::body::Body::from("代理内部错误"))
            .unwrap_or_else(|_| {
                let (mut parts, _) = Response::new(()).into_parts();
                parts.status = StatusCode::INTERNAL_SERVER_ERROR;
                Response::from_parts(parts, axum::body::Body::from("error"))
            })
    })
}

/// 转发非流式 JSON 响应。
pub(crate) async fn forward_json_response(upstream: reqwest::Response) -> Response {
    let status = upstream.status();
    let body = upstream.text().await.unwrap_or_default();
    build_response(
        status,
        &[("content-type", "application/json")],
        axum::body::Body::from(body),
    )
}

/// 原样转发 SSE 流。
pub(crate) fn forward_sse_stream(upstream: reqwest::Response) -> Response {
    let status = upstream.status();
    let stream = upstream
        .bytes_stream()
        .map(|chunk| chunk.map_err(|e| std::io::Error::other(format!("{e}"))));

    build_response(
        status,
        &[
            ("content-type", "text/event-stream"),
            ("cache-control", "no-cache"),
            ("connection", "keep-alive"),
        ],
        axum::body::Body::from_stream(stream),
    )
}
