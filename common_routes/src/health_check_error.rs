#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
pub(crate) enum HealthCheckError {
    #[error("service is unavailable")]
    Unavailable,
}
impl axum::response::IntoResponse for HealthCheckError {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::Unavailable => axum::response::IntoResponse::into_response(
                frontend_contract::api_problem_error::ApiProblemError::ServiceUnavailable,
            ),
        }
    }
}
