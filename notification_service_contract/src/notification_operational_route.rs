#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    proc_macro_frontend_contract_derive_route_catalog::RouteCatalog,
)]
#[route_catalog(
    family = NotificationOperationalRouteFamily,
    body_limit = crate::notification_api_body_max_bytes::NOTIFICATION_API_BODY_MAX_BYTES,
)]
pub enum NotificationOperationalRoute {
    #[route_catalog_route(
        contract = frontend_contract::route_contract::RouteContract::new(
            frontend_contract::authentication_requirement::AuthenticationRequirement::Public,
            frontend_contract::route_method::RouteMethod::Get,
            frontend_contract::mutation_kind::MutationKind::ReadOnly,
            frontend_contract::contract_str::ContractStr::from("/metrics/read"),
            frontend_contract::success_status::SuccessStatus::Code200,
        ),
        path = "/metrics/read",
        exclude_from_family,
    )]
    Metrics,
    #[route_catalog_route(
        contract = frontend_contract::route_contract::RouteContract::new(
            frontend_contract::authentication_requirement::AuthenticationRequirement::Public,
            frontend_contract::route_method::RouteMethod::Get,
            frontend_contract::mutation_kind::MutationKind::ReadOnly,
            frontend_contract::contract_str::ContractStr::from("/openapi.json/read"),
            frontend_contract::success_status::SuccessStatus::Code200,
        ),
        path = "/openapi.json/read",
        exclude_from_family,
    )]
    OpenApi,
}

impl frontend_contract::route_registration_contract::RouteRegistrationContract
    for NotificationOperationalRoute
{
    fn registration_method(self) -> frontend_contract::route_method::RouteMethod {
        self.contract().method()
    }
    fn registration_path(self) -> frontend_contract::contract_str::ContractStr {
        self.contract().path()
    }
}
