#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
use super::HealthCheckSucceeded;

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    optimal_memory_layout::OptimalMemoryLayout,
    newtype::FromInner,
)]
pub(crate) struct AxumHealthCheckStatus(pub(super) axum::http::StatusCode);
impl AxumHealthCheckStatus {
    pub(crate) fn is_ok(self) -> HealthCheckSucceeded {
        HealthCheckSucceeded::from(self.0 == axum::http::StatusCode::OK)
    }
}
impl axum::response::IntoResponse for AxumHealthCheckStatus {
    fn into_response(self) -> axum::response::Response {
        axum::response::IntoResponse::into_response(self.0)
    }
}
