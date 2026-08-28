pub(super) async fn readiness_report(
    app_state: &crate::ArcCommonRoutesAppState,
) -> Option<crate::JsonRes<crate::HealthReport>> {
    super::health_report_response::health_report_response(crate::HealthReport::readiness(
        crate::HealthDatabaseAvailable::from(bool::from(
            super::database_is_ready::database_is_ready(app_state).await,
        )),
    ))
}
