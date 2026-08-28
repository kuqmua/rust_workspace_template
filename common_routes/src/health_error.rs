use super::health_unavailable_response;

#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
pub(crate) enum HealthError {
    #[error("service is unavailable")]
    Unavailable,
}
impl axum::response::IntoResponse for HealthError {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::Unavailable => health_unavailable_response(),
        }
    }
}
