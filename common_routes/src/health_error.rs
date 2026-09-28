#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
pub(crate) enum HealthError {
    #[error("service is unavailable")]
    Unavailable(crate::health_report::HealthReport),
}
impl axum::response::IntoResponse for HealthError {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::Unavailable(health_report) => axum::response::IntoResponse::into_response((
                axum::http::StatusCode::SERVICE_UNAVAILABLE,
                axum::Json(health_report),
            )),
        }
    }
}
