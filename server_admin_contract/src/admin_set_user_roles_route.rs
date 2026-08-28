use super::admin_permission_requirement;

#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, frontend_contract::TypedRoute,
)]
#[typed_route(error_policy = frontend_contract::RouteErrorPolicy::Default, authentication = admin_permission_requirement(crate::domain_types::AdminPermission::UserRolesUpdate), method = frontend_contract::RouteMethod::Put, mutation = frontend_contract::RouteMutation::Mutating, obligations = frontend_contract::AUTHENTICATED_MUTATING_ROUTE_COVERAGE_OBLIGATIONS, openapi_operation_id = "set_user_roles", path = "/users/{user_id}/roles", path_parameter = crate::domain_types::AdminUserId, request = crate::domain_types::AdminSetUserRolesReq, request_body = frontend_contract::RouteRequestBody::Json, response = crate::domain_types::AdminNoBody, success_status = frontend_contract::SuccessStatus::Code204, transport = frontend_contract::AuthenticatedTransport)]
pub struct AdminSetUserRolesRoute;
