#[frontend_contract_macros::route_openapi(tag = "service")]
// The owner module retains lint-sensitive semantics from the original implementation.
#[allow(
    clippy::single_call_fn,
    reason = "route registry owns this Axum handler"
)]
pub(super) async fn health_live() -> Result<
    crate::json_res::JsonRes<crate::health_report::HealthReport>,
    crate::health_error::HealthError,
> {
    super::health_report_response::health_report_response(
        crate::health_report::HealthReport::liveness(),
    )
    .ok_or(crate::health_error::HealthError::Unavailable)
}
