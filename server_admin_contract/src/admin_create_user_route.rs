use super::admin_permission_requirement;

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    frontend_contract::domain_types::TypedRoute,
)]
#[typed_route(error_policy = frontend_contract::domain_types::RouteErrorPolicy::Default, authentication = admin_permission_requirement(crate::domain_types::AdminPermission::UsersCreate), method = frontend_contract::domain_types::RouteMethod::Post, mutation = frontend_contract::domain_types::RouteMutation::Mutating, obligations = frontend_contract::domain_types::AUTHENTICATED_MUTATING_ROUTE_COVERAGE_OBLIGATIONS, openapi_operation_id = "create_user", path = "/users", request = crate::domain_types::AdminCreateUserReq, request_body = frontend_contract::domain_types::RouteRequestBody::Json, response = crate::domain_types::AdminCreateUserRes, success_status = frontend_contract::domain_types::SuccessStatus::Code201, transport = frontend_contract::domain_types::AuthenticatedTransport)]
pub struct AdminCreateUserRoute;
