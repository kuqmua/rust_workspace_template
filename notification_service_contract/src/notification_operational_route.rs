use super::domain_types::NOTIFICATION_API_BODY_MAX_BYTES;

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    frontend_contract::RouteCatalog,
)]
#[route_catalog(
    family = NotificationOperationalRouteFamily,
    body_limit = NOTIFICATION_API_BODY_MAX_BYTES,
)]
pub enum NotificationOperationalRoute {
    #[route_catalog_route(
        contract = frontend_contract::RouteContract::new(
            frontend_contract::AuthenticationRequirement::Public,
            frontend_contract::RouteMethod::Get,
            frontend_contract::MutationKind::ReadOnly,
            frontend_contract::ContractStr::from("/metrics"),
            frontend_contract::SuccessStatus::Code200,
        ),
        path = "/metrics",
        exclude_from_family,
    )]
    Metrics,
    #[route_catalog_route(
        contract = frontend_contract::RouteContract::new(
            frontend_contract::AuthenticationRequirement::Public,
            frontend_contract::RouteMethod::Get,
            frontend_contract::MutationKind::ReadOnly,
            frontend_contract::ContractStr::from("/openapi.json"),
            frontend_contract::SuccessStatus::Code200,
        ),
        path = "/openapi.json",
        exclude_from_family,
    )]
    OpenApi,
}

impl frontend_contract::RouteRegistrationContract for NotificationOperationalRoute {
    fn method(self) -> frontend_contract::RouteMethod {
        frontend_contract::RouteMethod::Get
    }
    fn path(self) -> frontend_contract::RegisteredRoutePath {
        frontend_contract::RegisteredRoutePath::from(match self {
            Self::Metrics => constants_str::METRICS,
            Self::OpenApi => constants_str::OPENAPI_JSON,
        })
    }
}
