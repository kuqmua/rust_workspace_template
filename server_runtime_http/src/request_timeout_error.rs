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
                axum::Json(super::RequestTimeoutBody {
                    error: super::StdRequestTimeoutMessage::from(constants_str::REQUEST_TIMEOUT),
                }),
            )),
        }
    }
}
