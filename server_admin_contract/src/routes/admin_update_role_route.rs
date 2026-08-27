use super::admin_permission_requirement;

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    frontend_contract::domain_types::TypedRoute,
)]
#[typed_route(error_policy = frontend_contract::domain_types::RouteErrorPolicy::Default, authentication = admin_permission_requirement(crate::domain_types::AdminPermission::RolesUpdate), method = frontend_contract::domain_types::RouteMethod::Patch, mutation = frontend_contract::domain_types::RouteMutation::Mutating, obligations = frontend_contract::domain_types::AUTHENTICATED_MUTATING_ROUTE_COVERAGE_OBLIGATIONS, openapi_operation_id = "update_role", path = "/roles/{role_id}", path_parameter = crate::domain_types::AdminRoleId, request = crate::domain_types::AdminUpdateRoleReq, request_body = frontend_contract::domain_types::RouteRequestBody::Json, response = crate::domain_types::AdminNoBody, success_status = frontend_contract::domain_types::SuccessStatus::Code204, transport = frontend_contract::domain_types::AuthenticatedTransport)]
pub struct AdminUpdateRoleRoute;
