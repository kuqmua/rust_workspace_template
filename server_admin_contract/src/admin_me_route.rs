#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, frontend_contract::TypedRoute,
)]
#[typed_route(
    authentication = frontend_contract::AuthenticationRequirement::Authenticated,
    method = frontend_contract::RouteMethod::Get,
    mutation = frontend_contract::RouteMutation::ReadOnly,
    obligations = frontend_contract::AUTHENTICATED_READ_ROUTE_COVERAGE_OBLIGATIONS,
    error_policy = frontend_contract::RouteErrorPolicy::Default,
    openapi_operation_id = "me",
    path = "/auth/me",
    request = crate::domain_types::AdminNoBody,
    response = crate::domain_types::AuthenticatedAdmin,
    success_status = frontend_contract::SuccessStatus::Code200,
    transport = frontend_contract::AuthenticatedTransport,
)]
pub struct AdminMeRoute;
