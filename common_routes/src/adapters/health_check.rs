#[frontend_contract::domain_types::route_openapi(tag = "service")]
#[allow(clippy::single_call_fn)]
pub(super) async fn health_check(
    app_state: crate::domain_types::ArcCommonRoutesAppState,
) -> Result<crate::domain_types::AxumHealthCheckStatus, crate::domain_types::HealthCheckError> {
    let status = crate::domain_types::map_health_check_status(
        super::database_is_ready::database_is_ready(&app_state).await,
    );
    if bool::from(status.is_ok()) {
        Ok(status)
    } else {
        Err(crate::domain_types::HealthCheckError::Unavailable)
    }
}
