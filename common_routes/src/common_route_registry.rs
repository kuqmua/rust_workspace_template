proc_macro_frontend_contract::route_registry! {
    #[openapi(tags((name = "service", description = "Service operational routes")))]
    pub(super);
    state = crate::arc_common_routes_app_state::ArcCommonRoutesAppState,
    family = crate::common_route::CommonRouteFamily;
    ("", "");
    schemas(
        crate::health_component::HealthComponent,
        crate::health_component_kind::HealthComponentKind,
        crate::health_components::HealthComponents,
        crate::health_status::HealthStatus
    );
    (crate::git_info_route::GitInfoRoute, crate::git_info_response::git_info_response),
    (crate::health_route::HealthRoute, crate::health::health),
    (crate::health_check_route::HealthCheckRoute, crate::health_check::health_check),
    (crate::health_live_route::HealthLiveRoute, crate::health_live::health_live),
    (crate::health_ready_route::HealthReadyRoute, crate::health_ready::health_ready),
}
