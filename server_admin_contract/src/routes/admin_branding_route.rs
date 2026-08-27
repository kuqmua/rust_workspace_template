#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    frontend_contract::domain_types::TypedRoute,
)]
#[typed_route(error_policy = frontend_contract::domain_types::RouteErrorPolicy::Default, authentication = frontend_contract::domain_types::AuthenticationRequirement::Public, method = frontend_contract::domain_types::RouteMethod::Get, mutation = frontend_contract::domain_types::RouteMutation::ReadOnly, obligations = frontend_contract::domain_types::PUBLIC_READ_ROUTE_COVERAGE_OBLIGATIONS, openapi_operation_id = "branding", path = "/branding", request = crate::domain_types::AdminNoBody, response = crate::domain_types::AdminBrandingView, success_status = frontend_contract::domain_types::SuccessStatus::Code200, transport = frontend_contract::domain_types::PublicTransport)]
pub struct AdminBrandingRoute;
