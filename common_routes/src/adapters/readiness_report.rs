pub(super) async fn readiness_report(
    app_state: &crate::domain_types::ArcCommonRoutesAppState,
) -> Option<crate::domain_types::JsonRes<crate::domain_types::HealthReport>> {
    super::health_report_response::health_report_response(
        crate::domain_types::HealthReport::readiness(
            crate::domain_types::HealthDatabaseAvailable::from(bool::from(
                super::database_is_ready::database_is_ready(app_state).await,
            )),
        ),
    )
}
