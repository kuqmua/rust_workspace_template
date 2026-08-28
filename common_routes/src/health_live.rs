#[frontend_contract::route_openapi(tag = "service")]
// The owner module retains lint-sensitive semantics from the original implementation.
#[allow(
    clippy::single_call_fn,
    reason = "route registry owns this Axum handler"
)]
pub(super) async fn health_live() -> Result<crate::JsonRes<crate::HealthReport>, crate::HealthError>
{
    super::health_report_response::health_report_response(crate::HealthReport::liveness())
        .ok_or(crate::HealthError::Unavailable)
}
