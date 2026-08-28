#[frontend_contract::route_openapi(tag = "service")]
// The owner module retains lint-sensitive semantics from the original implementation.
#[allow(
    clippy::single_call_fn,
    reason = "route registry owns this Axum handler"
)]
pub(super) async fn health_check(
    app_state: crate::ArcCommonRoutesAppState,
) -> Result<crate::AxumHealthCheckStatus, crate::HealthCheckError> {
    let status = if super::database_is_ready::database_is_ready(&app_state)
        .await
        .0
    {
        crate::AxumHealthCheckStatus::from(axum::http::StatusCode::OK)
    } else {
        crate::AxumHealthCheckStatus::from(axum::http::StatusCode::SERVICE_UNAVAILABLE)
    };
    if bool::from(status.is_ok()) {
        Ok(status)
    } else {
        Err(crate::HealthCheckError::Unavailable)
    }
}
