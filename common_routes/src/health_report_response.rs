pub(super) fn health_report_response(
    report: crate::HealthReport,
) -> Option<crate::JsonRes<crate::HealthReport>> {
    match report.status() {
        crate::HealthStatus::Ok => Some(crate::make_json_response(report)),
        crate::HealthStatus::Degraded | crate::HealthStatus::Error => None,
    }
}
