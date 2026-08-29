#[frontend_contract_macros::route_openapi(tag = "service")]
// The owner module retains lint-sensitive semantics from the original implementation.
#[allow(
    clippy::single_call_fn,
    reason = "route registry owns this Axum handler"
)]
pub(super) async fn health(
    app_state: crate::arc_common_routes_app_state::ArcCommonRoutesAppState,
) -> Result<
    crate::json_res::JsonRes<crate::health_report::HealthReport>,
    crate::health_error::HealthError,
> {
    super::readiness_report::readiness_report(&app_state)
        .await
        .ok_or(crate::health_error::HealthError::Unavailable)
}
