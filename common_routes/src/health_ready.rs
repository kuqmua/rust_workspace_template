#[frontend_contract::route_openapi(tag = "service")]
// The owner module retains lint-sensitive semantics from the original implementation.
#[allow(
    clippy::single_call_fn,
    reason = "route registry owns this Axum handler"
)]
pub(super) async fn health_ready(
    app_state: crate::ArcCommonRoutesAppState,
) -> Result<crate::JsonRes<crate::HealthReport>, crate::HealthError> {
    super::readiness_report::readiness_report(&app_state)
        .await
        .ok_or(crate::HealthError::Unavailable)
}
