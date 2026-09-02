#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    proc_macro_frontend_contract::TypedRoute,
)]
#[typed_route(error_policy = frontend_contract::route_error_policy::RouteErrorPolicy::Default, authentication = crate::admin_permission_requirement::admin_permission_requirement(crate::admin_permission::AdminPermission::SystemSettingsUpdate), method = frontend_contract::route_method::RouteMethod::Patch, mutation = frontend_contract::route_mutation::RouteMutation::Mutating, obligations = frontend_contract::route_coverage_obligation::AUTHENTICATED_MUTATING_ROUTE_COVERAGE_OBLIGATIONS, openapi_operation_id = "update_settings", path = "/system_settings", request = crate::admin_update_settings_request::AdminUpdateSettingsRequest, request_body = frontend_contract::route_request_body::RouteRequestBody::Json, response = crate::admin_no_body::AdminNoBody, success_status = frontend_contract::success_status::SuccessStatus::Code204, transport = frontend_contract::authenticated_transport::AuthenticatedTransport)]
pub struct AdminUpdateSettingsRoute;
