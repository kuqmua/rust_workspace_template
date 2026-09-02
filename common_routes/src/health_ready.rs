#[proc_macro_frontend_contract::route_openapi(tag = "service")]
// The owner module retains lint-sensitive semantics from the original implementation.
#[allow(
    clippy::single_call_fn,
    reason = "route registry owns this Axum handler"
)]
pub(super) async fn health_ready(
    arc_common_routes_app_state: crate::arc_common_routes_app_state::ArcCommonRoutesAppState,
) -> Result<
    crate::json_response::JsonResponse<crate::health_report::HealthReport>,
    crate::health_error::HealthError,
> {
    super::readiness_report::readiness_report(&arc_common_routes_app_state)
        .await
        .ok_or(crate::health_error::HealthError::Unavailable)
}
