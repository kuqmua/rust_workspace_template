#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    proc_macro_frontend_contract_derive_typed_route::TypedRoute,
)]
#[typed_route(error_policy = frontend_contract::route_error_policy::RouteErrorPolicy::Default, authentication = crate::admin_permission_requirement::admin_permission_requirement(crate::admin_permission::AdminPermission::RolesRead), method = frontend_contract::route_method::RouteMethod::Post, mutation = frontend_contract::route_mutation::RouteMutation::ReadOnly, obligations = frontend_contract::route_coverage_obligation::AUTHENTICATED_READ_ROUTE_COVERAGE_OBLIGATIONS, openapi_operation_id = "read_roles", path = "/roles/read", request = crate::admin_roles_read_request::AdminRolesReadRequest, request_body = frontend_contract::route_request_body::RouteRequestBody::Json, response = crate::admin_roles_page::AdminRolesPage, success_status = frontend_contract::success_status::SuccessStatus::Code200, transport = frontend_contract::authenticated_transport::AuthenticatedTransport)]
pub struct AdminReadRolesRoute;
