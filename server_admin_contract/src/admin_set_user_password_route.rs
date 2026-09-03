#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    proc_macro_frontend_contract_derive_typed_route::TypedRoute,
)]
#[typed_route(error_policy = frontend_contract::route_error_policy::RouteErrorPolicy::Default, authentication = crate::admin_permission_requirement::admin_permission_requirement(crate::admin_permission::AdminPermission::UsersUpdate), method = frontend_contract::route_method::RouteMethod::Post, mutation = frontend_contract::route_mutation::RouteMutation::Mutating, obligations = frontend_contract::route_coverage_obligation::AUTHENTICATED_MUTATING_ROUTE_COVERAGE_OBLIGATIONS, openapi_operation_id = "set_user_password", path = "/users/{user_id}/password", path_parameter = crate::admin_user_id::AdminUserId, request = crate::admin_set_user_password_request::AdminSetUserPasswordRequest, request_body = frontend_contract::route_request_body::RouteRequestBody::Json, response = crate::admin_no_body::AdminNoBody, success_status = frontend_contract::success_status::SuccessStatus::Code204, transport = frontend_contract::authenticated_transport::AuthenticatedTransport)]
pub struct AdminSetUserPasswordRoute;
