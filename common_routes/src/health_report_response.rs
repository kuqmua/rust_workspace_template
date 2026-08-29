pub(super) fn health_report_response(
    report: crate::health_report::HealthReport,
) -> Option<crate::json_res::JsonRes<crate::health_report::HealthReport>> {
    match report.status() {
        crate::health_status::HealthStatus::Ok => {
            Some(crate::make_json_response::make_json_response(report))
        }
        crate::health_status::HealthStatus::Degraded
        | crate::health_status::HealthStatus::Error => None,
    }
}
