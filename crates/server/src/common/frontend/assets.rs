use axum::{
    body::Body,
    http::{header, StatusCode, Uri},
    response::Response,
};
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "../../frontend/dist/"]
struct FrontendAssets;

pub async fn serve_frontend(uri: Uri) -> Response {
    let path = uri.path();
    let asset_path = path.trim_start_matches('/');

    if asset_path.starts_with("api/")
        || asset_path.starts_with("pool/")
        || asset_path.starts_with("go/")
    {
        return response(StatusCode::NOT_FOUND, None, Body::from("未找到"));
    }

    let asset_path = match asset_path.is_empty() {
        true => "index.html",
        false => asset_path,
    };

    match FrontendAssets::get(asset_path) {
        Some(file) => response(
            StatusCode::OK,
            Some(mime_from_path(asset_path)),
            Body::from(file.data),
        ),
        None => FrontendAssets::get("index.html").map_or_else(
            || response(StatusCode::NOT_FOUND, None, Body::from("未找到")),
            |index| {
                response(
                    StatusCode::OK,
                    Some("text/html; charset=utf-8"),
                    Body::from(index.data),
                )
            },
        ),
    }
}

fn response(status: StatusCode, content_type: Option<&'static str>, body: Body) -> Response {
    let mut response = Response::new(body);
    *response.status_mut() = status;
    if let Some(content_type) = content_type {
        response
            .headers_mut()
            .insert(header::CONTENT_TYPE, header_value(content_type));
    }
    response
}

fn header_value(value: &'static str) -> header::HeaderValue {
    header::HeaderValue::from_static(value)
}

fn mime_from_path(path: &str) -> &'static str {
    match path.rsplit('.').next() {
        Some("html") => "text/html; charset=utf-8",
        Some("css") => "text/css; charset=utf-8",
        Some("js" | "mjs") => "application/javascript; charset=utf-8",
        Some("json") => "application/json",
        Some("png") => "image/png",
        Some("svg") => "image/svg+xml",
        Some("ico") => "image/x-icon",
        Some("woff2") => "font/woff2",
        _ => "application/octet-stream",
    }
}
