#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, frontend_contract::TypedRoute,
)]
#[typed_route(error_policy = frontend_contract::RouteErrorPolicy::Default, authentication = frontend_contract::AuthenticationRequirement::Public, method = frontend_contract::RouteMethod::Get, mutation = frontend_contract::RouteMutation::ReadOnly, obligations = frontend_contract::PUBLIC_READ_ROUTE_COVERAGE_OBLIGATIONS, openapi_operation_id = "branding", path = "/branding", request = crate::domain_types::AdminNoBody, response = crate::domain_types::AdminBrandingView, success_status = frontend_contract::SuccessStatus::Code200, transport = frontend_contract::PublicTransport)]
pub struct AdminBrandingRoute;
