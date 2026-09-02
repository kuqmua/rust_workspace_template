pub(super) async fn readiness_report(
    arc_common_routes_app_state: &crate::arc_common_routes_app_state::ArcCommonRoutesAppState,
) -> Option<crate::json_response::JsonResponse<crate::health_report::HealthReport>> {
    super::health_report_response::health_report_response(
        crate::health_report::HealthReport::readiness(
            crate::health_database_available::HealthDatabaseAvailable::from(bool::from(
                super::database_is_ready::database_is_ready(arc_common_routes_app_state).await,
            )),
        ),
    )
}
