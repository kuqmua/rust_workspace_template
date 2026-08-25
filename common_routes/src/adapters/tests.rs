#[test]
fn health_report_response_only_returns_healthy_reports() {
    assert!(super::health_report_response(crate::domain_types::HealthReport::liveness()).is_some());
    assert!(
        super::health_report_response(crate::domain_types::HealthReport::readiness(
            crate::domain_types::HealthDatabaseAvailable::from(false),
        ))
        .is_none()
    );
}
