#[proc_macro_frontend_contract::route_openapi(tag = "service")]
// The owner module retains lint-sensitive semantics from the original implementation.
#[allow(
    clippy::single_call_fn,
    reason = "route registry owns this Axum handler"
)]
pub(super) async fn health_check(
    app_state: crate::arc_common_routes_app_state::ArcCommonRoutesAppState,
) -> Result<
    crate::axum_health_check_status::AxumHealthCheckStatus,
    crate::health_check_error::HealthCheckError,
> {
    let status = if bool::from(super::database_is_ready::database_is_ready(&app_state).await) {
        crate::axum_health_check_status::AxumHealthCheckStatus::from(axum::http::StatusCode::OK)
    } else {
        crate::axum_health_check_status::AxumHealthCheckStatus::from(
            axum::http::StatusCode::SERVICE_UNAVAILABLE,
        )
    };
    if bool::from(status.is_ok()) {
        Ok(status)
    } else {
        Err(crate::health_check_error::HealthCheckError::Unavailable)
    }
}
