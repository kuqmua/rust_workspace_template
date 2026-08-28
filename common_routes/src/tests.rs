#[test]
fn common_routes_tests() {
    assert!(super::health_report_response(crate::domain_types::HealthReport::liveness()).is_some());
    assert!(
        super::health_report_response(crate::domain_types::HealthReport::readiness(
            crate::domain_types::HealthDatabaseAvailable::from(false),
        ))
        .is_none()
    );
}
