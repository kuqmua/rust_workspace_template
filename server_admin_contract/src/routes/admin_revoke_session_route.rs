#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    frontend_contract::domain_types::TypedRoute,
)]
#[typed_route(error_policy = frontend_contract::domain_types::RouteErrorPolicy::Default, authentication = frontend_contract::domain_types::AuthenticationRequirement::Authenticated, method = frontend_contract::domain_types::RouteMethod::Delete, mutation = frontend_contract::domain_types::RouteMutation::Mutating, obligations = frontend_contract::domain_types::AUTHENTICATED_MUTATING_ROUTE_COVERAGE_OBLIGATIONS, openapi_operation_id = "revoke_session", path = "/auth/sessions/{session_id}", path_parameter = crate::domain_types::AdminSessionIdentifier, request = crate::domain_types::AdminNoBody, response = crate::domain_types::AdminNoBody, success_status = frontend_contract::domain_types::SuccessStatus::Code204, transport = frontend_contract::domain_types::AuthenticatedTransport)]
pub struct AdminRevokeSessionRoute;
