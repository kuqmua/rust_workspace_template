#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    proc_macro_newtype_from_inner::FromInner,
)]
pub(crate) struct AxumHealthCheckStatus(axum::http::StatusCode);
impl AxumHealthCheckStatus {
    pub(crate) fn is_ok(self) -> crate::health_check_succeeded::HealthCheckSucceeded {
        crate::health_check_succeeded::HealthCheckSucceeded::from(
            self.0 == axum::http::StatusCode::OK,
        )
    }
}
impl axum::response::IntoResponse for AxumHealthCheckStatus {
    fn into_response(self) -> axum::response::Response {
        axum::response::IntoResponse::into_response(self.0)
    }
}
