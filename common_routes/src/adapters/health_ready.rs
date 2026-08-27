#[frontend_contract::domain_types::route_openapi(tag = "service")]
#[allow(clippy::single_call_fn)]
pub(super) async fn health_ready(
    app_state: crate::domain_types::ArcCommonRoutesAppState,
) -> Result<
    crate::domain_types::JsonRes<crate::domain_types::HealthReport>,
    crate::domain_types::HealthReadyError,
> {
    super::readiness_report::readiness_report(&app_state)
        .await
        .ok_or(crate::domain_types::HealthReadyError::Unavailable)
}
