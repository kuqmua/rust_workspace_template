#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    frontend_contract_macros::TypedRoute,
)]
#[typed_route(error_policy = frontend_contract::route_error_policy::RouteErrorPolicy::Default, authentication = crate::admin_permission_requirement::admin_permission_requirement(crate::admin_permission::AdminPermission::UsersRead), method = frontend_contract::route_method::RouteMethod::Get, mutation = frontend_contract::route_mutation::RouteMutation::ReadOnly, obligations = frontend_contract::route_coverage_obligation::AUTHENTICATED_READ_ROUTE_COVERAGE_OBLIGATIONS, openapi_operation_id = "list_users", path = "/users", request = crate::admin_no_body::AdminNoBody, response = crate::admin_users_page::AdminUsersPage, success_status = frontend_contract::success_status::SuccessStatus::Code200, transport = frontend_contract::authenticated_transport::AuthenticatedTransport)]
pub struct AdminListUsersRoute;
