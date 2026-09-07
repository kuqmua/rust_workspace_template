#[test]
fn test_common_routes_tests() {
    assert!(
        crate::health_report_response::health_report_response(
            crate::health_report::HealthReport::liveness()
        )
        .is_some()
    );
    assert!(
        crate::health_report_response::health_report_response(
            crate::health_report::HealthReport::readiness(
                crate::health_database_available::HealthDatabaseAvailable::from(false),
            )
        )
        .is_none()
    );
}
