#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy)]
struct ClientTransport;

impl frontend_contract::Transport for ClientTransport {
    fn send(
        &self,
        _request: frontend_contract::TransportRequest,
    ) -> impl Future<
        Output = Result<frontend_contract::TransportResponse, frontend_contract::TransportError>,
    > + '_ {
        std::future::ready(Err(frontend_contract::TransportError::default()))
    }
}

#[test]
fn every_route_has_named_route_and_client_functions() {
    assert_eq!(
        <crate::CommonRouteFamily as frontend_contract::RouteFamily>::ROUTE_COUNT,
        5usize
    );
    assert_eq!(crate::git_info_route(), crate::CommonRoute::GitInfo.path());
    assert_eq!(crate::health_route(), crate::CommonRoute::Health.path());
    assert_eq!(
        crate::health_check_route(),
        crate::CommonRoute::HealthCheck.path()
    );
    assert_eq!(
        crate::health_live_route(),
        crate::CommonRoute::HealthLive.path()
    );
    assert_eq!(
        crate::health_ready_route(),
        crate::CommonRoute::HealthReady.path()
    );
    assert_eq!(
        size_of_val(&crate::git_info_client::<ClientTransport>),
        constants_usize::ZERO
    );
    assert_eq!(
        size_of_val(&crate::health_client::<ClientTransport>),
        constants_usize::ZERO
    );
    assert_eq!(
        size_of_val(&crate::health_check_client::<ClientTransport>),
        constants_usize::ZERO
    );
    assert_eq!(
        size_of_val(&crate::health_live_client::<ClientTransport>),
        constants_usize::ZERO
    );
    assert_eq!(
        size_of_val(&crate::health_ready_client::<ClientTransport>),
        constants_usize::ZERO
    );
}

#[test]
fn paths_use_snake_case_segments() {
    assert!(!constants_str::COMMON_ROUTES_SWAGGER_UI.contains('-'));
    crate::CommonRoute::ALL.into_iter().for_each(|route| {
        assert!(!route.path().as_ref().contains('-'));
    });
}

#[test]
fn family_coverage_is_complete() {
    let descriptors =
        <crate::CommonRouteFamily as frontend_contract::RouteFamily>::coverage_descriptors();
    assert_eq!(
        frontend_contract::validate_route_coverage(descriptors.as_ref()),
        Ok(())
    );
    assert_eq!(descriptors.as_ref().len(), crate::CommonRoute::ALL.len());
}
