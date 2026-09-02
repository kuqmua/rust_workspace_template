pub(super) fn health_report_response(
    health_report: crate::health_report::HealthReport,
) -> Option<crate::json_response::JsonResponse<crate::health_report::HealthReport>> {
    match health_report.status() {
        crate::health_status::HealthStatus::Ok => {
            Some(crate::make_json_response::make_json_response(health_report))
        }
        crate::health_status::HealthStatus::Degraded
        | crate::health_status::HealthStatus::Error => None,
    }
}
