use super::admin_permission_requirement;

#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, frontend_contract::TypedRoute,
)]
#[typed_route(error_policy = frontend_contract::RouteErrorPolicy::Default, authentication = admin_permission_requirement(crate::domain_types::AdminPermission::UsersRead), method = frontend_contract::RouteMethod::Get, mutation = frontend_contract::RouteMutation::ReadOnly, obligations = frontend_contract::AUTHENTICATED_READ_ROUTE_COVERAGE_OBLIGATIONS, openapi_operation_id = "list_users", path = "/users", request = crate::domain_types::AdminNoBody, response = crate::domain_types::AdminUsersPage, success_status = frontend_contract::SuccessStatus::Code200, transport = frontend_contract::AuthenticatedTransport)]
pub struct AdminListUsersRoute;
