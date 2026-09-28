#[allow(
    clippy::single_call_fn,
    reason = "named report mapper supports direct unit tests without a database client"
)]
pub(super) fn health_report_response(
    health_report: crate::health_report::HealthReport,
) -> Result<
    crate::json_response::JsonResponse<crate::health_report::HealthReport>,
    crate::health_error::HealthError,
> {
    match health_report.status() {
        crate::health_status::HealthStatus::Ok => {
            Ok(crate::make_json_response::make_json_response(health_report))
        }
        crate::health_status::HealthStatus::Degraded
        | crate::health_status::HealthStatus::Error => {
            Err(crate::health_error::HealthError::Unavailable(health_report))
        }
    }
}
