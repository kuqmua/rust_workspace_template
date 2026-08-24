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
        <super::super::CommonRouteFamily as frontend_contract::RouteFamily>::ROUTE_COUNT,
        5usize
    );
    assert_eq!(
        super::super::git_info_route(),
        super::super::CommonRoute::GitInfo.path()
    );
    assert_eq!(
        super::super::health_route(),
        super::super::CommonRoute::Health.path()
    );
    assert_eq!(
        super::super::health_check_route(),
        super::super::CommonRoute::HealthCheck.path()
    );
    assert_eq!(
        super::super::health_live_route(),
        super::super::CommonRoute::HealthLive.path()
    );
    assert_eq!(
        super::super::health_ready_route(),
        super::super::CommonRoute::HealthReady.path()
    );
    assert_eq!(
        size_of_val(&super::super::git_info_client::<ClientTransport>),
        usize_constants::ZERO
    );
    assert_eq!(
        size_of_val(&super::super::health_client::<ClientTransport>),
        usize_constants::ZERO
    );
    assert_eq!(
        size_of_val(&super::super::health_check_client::<ClientTransport>),
        usize_constants::ZERO
    );
    assert_eq!(
        size_of_val(&super::super::health_live_client::<ClientTransport>),
        usize_constants::ZERO
    );
    assert_eq!(
        size_of_val(&super::super::health_ready_client::<ClientTransport>),
        usize_constants::ZERO
    );
}

#[test]
fn paths_use_snake_case_segments() {
    assert!(!str_constants::COMMON_ROUTES_SWAGGER_UI.contains('-'));
    super::super::CommonRoute::ALL
        .into_iter()
        .for_each(|route| {
            assert!(!route.path().as_ref().contains('-'));
        });
}

#[test]
fn family_coverage_is_complete() {
    let descriptors =
        <super::super::CommonRouteFamily as frontend_contract::RouteFamily>::coverage_descriptors();
    assert_eq!(
        frontend_contract::validate_route_coverage(descriptors.as_ref()),
        Ok(())
    );
    assert_eq!(
        descriptors.as_ref().len(),
        super::super::CommonRoute::ALL.len()
    );
}
