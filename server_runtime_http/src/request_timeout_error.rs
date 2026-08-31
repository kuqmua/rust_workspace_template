#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
pub(super) enum RequestTimeoutError {
    #[error("request timeout")]
    TimedOut,
}

impl axum::response::IntoResponse for RequestTimeoutError {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::TimedOut => axum::response::IntoResponse::into_response((
                http::StatusCode::SERVICE_UNAVAILABLE,
                axum::Json(crate::request_timeout_body::RequestTimeoutBody::new(
                    crate::std_request_timeout_message::StdRequestTimeoutMessage::from(
                        constants_str::REQUEST_TIMEOUT,
                    ),
                )),
            )),
        }
    }
}
