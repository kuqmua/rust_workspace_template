#[test]
fn common_routes_tests() {
    assert!(super::health_report_response(crate::HealthReport::liveness()).is_some());
    assert!(
        super::health_report_response(crate::HealthReport::readiness(
            crate::HealthDatabaseAvailable::from(false),
        ))
        .is_none()
    );
}
