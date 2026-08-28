#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, frontend_contract::TypedRoute,
)]
#[typed_route(error_policy = frontend_contract::RouteErrorPolicy::Default, authentication = frontend_contract::AuthenticationRequirement::Authenticated, method = frontend_contract::RouteMethod::Delete, mutation = frontend_contract::RouteMutation::Mutating, obligations = frontend_contract::AUTHENTICATED_MUTATING_ROUTE_COVERAGE_OBLIGATIONS, openapi_operation_id = "revoke_all_sessions", path = "/auth/sessions", request = crate::domain_types::AdminNoBody, response = crate::domain_types::AdminNoBody, success_status = frontend_contract::SuccessStatus::Code204, transport = frontend_contract::AuthenticatedTransport)]
pub struct AdminRevokeAllSessionsRoute;
