#[frontend_contract::domain_types::route_openapi(tag = "service")]
// The owner module retains lint-sensitive semantics from the original implementation.
#[allow(clippy::single_call_fn)]
pub(super) async fn health_live() -> Result<
    crate::domain_types::JsonRes<crate::domain_types::HealthReport>,
    crate::domain_types::HealthLiveError,
> {
    super::health_report_response::health_report_response(
        crate::domain_types::HealthReport::liveness(),
    )
    .ok_or(crate::domain_types::HealthLiveError::Unavailable)
}
