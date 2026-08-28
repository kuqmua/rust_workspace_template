use super::admin_permission_requirement;

#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, frontend_contract::TypedRoute,
)]
#[typed_route(error_policy = frontend_contract::RouteErrorPolicy::Default, authentication = admin_permission_requirement(crate::domain_types::AdminPermission::UsersUpdate), method = frontend_contract::RouteMethod::Patch, mutation = frontend_contract::RouteMutation::Mutating, obligations = frontend_contract::AUTHENTICATED_MUTATING_ROUTE_COVERAGE_OBLIGATIONS, openapi_operation_id = "update_user", path = "/users/{user_id}", path_parameter = crate::domain_types::AdminUserId, request = crate::domain_types::AdminUpdateUserReq, request_body = frontend_contract::RouteRequestBody::Json, response = crate::domain_types::AdminNoBody, success_status = frontend_contract::SuccessStatus::Code204, transport = frontend_contract::AuthenticatedTransport)]
pub struct AdminUpdateUserRoute;
