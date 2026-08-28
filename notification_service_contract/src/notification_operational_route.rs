use crate::domain_types::NOTIFICATION_API_BODY_MAX_BYTES;

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    frontend_contract::domain_types::RouteCatalog,
)]
#[route_catalog(
    family = NotificationOperationalRouteFamily,
    body_limit = NOTIFICATION_API_BODY_MAX_BYTES,
)]
pub enum NotificationOperationalRoute {
    #[route_catalog_route(
        contract = frontend_contract::domain_types::RouteContract::new(
            frontend_contract::domain_types::AuthenticationRequirement::Public,
            frontend_contract::domain_types::RouteMethod::Get,
            frontend_contract::domain_types::MutationKind::ReadOnly,
            frontend_contract::domain_types::ContractStr::from("/metrics"),
            frontend_contract::domain_types::SuccessStatus::Code200,
        ),
        path = "/metrics",
        exclude_from_family,
    )]
    Metrics,
    #[route_catalog_route(
        contract = frontend_contract::domain_types::RouteContract::new(
            frontend_contract::domain_types::AuthenticationRequirement::Public,
            frontend_contract::domain_types::RouteMethod::Get,
            frontend_contract::domain_types::MutationKind::ReadOnly,
            frontend_contract::domain_types::ContractStr::from("/openapi.json"),
            frontend_contract::domain_types::SuccessStatus::Code200,
        ),
        path = "/openapi.json",
        exclude_from_family,
    )]
    OpenApi,
}

impl frontend_contract::domain_types::RouteRegistrationContract for NotificationOperationalRoute {
    fn method(self) -> frontend_contract::domain_types::RouteMethod {
        frontend_contract::domain_types::RouteMethod::Get
    }
    fn path(self) -> frontend_contract::domain_types::RegisteredRoutePath {
        frontend_contract::domain_types::RegisteredRoutePath::from(match self {
            Self::Metrics => constants_str::METRICS,
            Self::OpenApi => constants_str::OPENAPI_JSON,
        })
    }
}
