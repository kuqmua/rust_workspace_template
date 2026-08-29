pub(super) async fn readiness_report(
    app_state: &crate::arc_common_routes_app_state::ArcCommonRoutesAppState,
) -> Option<crate::json_res::JsonRes<crate::health_report::HealthReport>> {
    super::health_report_response::health_report_response(
        crate::health_report::HealthReport::readiness(
            crate::health_database_available::HealthDatabaseAvailable::from(bool::from(
                super::database_is_ready::database_is_ready(app_state).await,
            )),
        ),
    )
}
