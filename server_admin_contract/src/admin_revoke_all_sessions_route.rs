#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    frontend_contract_macros::TypedRoute,
)]
#[typed_route(error_policy = frontend_contract::route_error_policy::RouteErrorPolicy::Default, authentication = frontend_contract::authentication_requirement::AuthenticationRequirement::Authenticated, method = frontend_contract::route_method::RouteMethod::Delete, mutation = frontend_contract::route_mutation::RouteMutation::Mutating, obligations = frontend_contract::route_coverage_obligation::AUTHENTICATED_MUTATING_ROUTE_COVERAGE_OBLIGATIONS, openapi_operation_id = "revoke_all_sessions", path = "/auth/sessions", request = crate::admin_no_body::AdminNoBody, response = crate::admin_no_body::AdminNoBody, success_status = frontend_contract::success_status::SuccessStatus::Code204, transport = frontend_contract::authenticated_transport::AuthenticatedTransport)]
pub struct AdminRevokeAllSessionsRoute;
