use super::admin_permission_requirement;

#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, frontend_contract::TypedRoute,
)]
#[typed_route(error_policy = frontend_contract::RouteErrorPolicy::Default, authentication = admin_permission_requirement(crate::domain_types::AdminPermission::RolesCreate), method = frontend_contract::RouteMethod::Post, mutation = frontend_contract::RouteMutation::Mutating, obligations = frontend_contract::AUTHENTICATED_MUTATING_ROUTE_COVERAGE_OBLIGATIONS, openapi_operation_id = "create_role", path = "/roles", request = crate::domain_types::AdminCreateRoleReq, request_body = frontend_contract::RouteRequestBody::Json, response = crate::domain_types::AdminCreateRoleRes, success_status = frontend_contract::SuccessStatus::Code201, transport = frontend_contract::AuthenticatedTransport)]
pub struct AdminCreateRoleRoute;
