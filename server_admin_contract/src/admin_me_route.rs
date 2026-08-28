#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    frontend_contract::domain_types::TypedRoute,
)]
#[typed_route(
    authentication = frontend_contract::domain_types::AuthenticationRequirement::Authenticated,
    method = frontend_contract::domain_types::RouteMethod::Get,
    mutation = frontend_contract::domain_types::RouteMutation::ReadOnly,
    obligations = frontend_contract::domain_types::AUTHENTICATED_READ_ROUTE_COVERAGE_OBLIGATIONS,
    error_policy = frontend_contract::domain_types::RouteErrorPolicy::Default,
    openapi_operation_id = "me",
    path = "/auth/me",
    request = crate::domain_types::AdminNoBody,
    response = crate::domain_types::AuthenticatedAdmin,
    success_status = frontend_contract::domain_types::SuccessStatus::Code200,
    transport = frontend_contract::domain_types::AuthenticatedTransport,
)]
pub struct AdminMeRoute;
