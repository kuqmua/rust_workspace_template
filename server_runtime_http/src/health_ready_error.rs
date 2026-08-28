#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
pub(super) enum HealthReadyError {
    #[error("service is unavailable")]
    Unavailable(super::HealthSnapshot),
}

impl axum::response::IntoResponse for HealthReadyError {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::Unavailable(snapshot) => axum::response::IntoResponse::into_response((
                http::StatusCode::SERVICE_UNAVAILABLE,
                axum::Json(snapshot),
            )),
        }
    }
}
