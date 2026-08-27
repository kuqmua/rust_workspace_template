pub(super) fn health_report_response(
    report: crate::domain_types::HealthReport,
) -> Option<crate::domain_types::JsonRes<crate::domain_types::HealthReport>> {
    match report.status() {
        crate::domain_types::HealthStatus::Ok => {
            Some(crate::domain_types::make_json_response(report))
        }
        crate::domain_types::HealthStatus::Degraded | crate::domain_types::HealthStatus::Error => {
            None
        }
    }
}
