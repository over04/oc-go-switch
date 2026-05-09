use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;

/// OpenAI 协议错误响应格式: `{"error": {"message": "...", "type": "...", ...}}`
pub(crate) fn openai_error(
    status: StatusCode,
    message: impl Into<String>,
    error_type: impl Into<String>,
    param: Option<&str>,
    code: Option<&str>,
) -> Response {
    #[derive(Serialize)]
    struct OpenAIError {
        message: String,
        #[serde(rename = "type")]
        error_type: String,
        param: Option<String>,
        code: Option<String>,
    }

    #[derive(Serialize)]
    struct Body {
        error: OpenAIError,
    }

    (status, Json(Body {
        error: OpenAIError {
            message: message.into(),
            error_type: error_type.into(),
            param: param.map(String::from),
            code: code.map(String::from),
        },
    })).into_response()
}

/// Anthropic 协议错误响应格式: `{"type": "error", "error": {"type": "...", "message": "..."}}`
pub(crate) fn anthropic_error(
    status: StatusCode,
    message: impl Into<String>,
    error_type: impl Into<String>,
) -> Response {
    #[derive(Serialize)]
    struct AnthropicErrorDetail {
        #[serde(rename = "type")]
        error_type: String,
        message: String,
    }

    #[derive(Serialize)]
    struct Body {
        #[serde(rename = "type")]
        body_type: &'static str,
        error: AnthropicErrorDetail,
    }

    (status, Json(Body {
        body_type: "error",
        error: AnthropicErrorDetail {
            error_type: error_type.into(),
            message: message.into(),
        },
    })).into_response()
}
