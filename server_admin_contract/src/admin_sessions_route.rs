#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    frontend_contract::domain_types::TypedRoute,
)]
#[typed_route(error_policy = frontend_contract::domain_types::RouteErrorPolicy::Default, authentication = frontend_contract::domain_types::AuthenticationRequirement::Authenticated, method = frontend_contract::domain_types::RouteMethod::Get, mutation = frontend_contract::domain_types::RouteMutation::ReadOnly, obligations = frontend_contract::domain_types::AUTHENTICATED_READ_ROUTE_COVERAGE_OBLIGATIONS, openapi_operation_id = "sessions", path = "/auth/sessions", request = crate::domain_types::AdminNoBody, response = crate::domain_types::AdminSessionsPage, success_status = frontend_contract::domain_types::SuccessStatus::Code200, transport = frontend_contract::domain_types::AuthenticatedTransport)]
pub struct AdminSessionsRoute;
