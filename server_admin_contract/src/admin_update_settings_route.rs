use super::admin_permission_requirement;

#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, frontend_contract::TypedRoute,
)]
#[typed_route(error_policy = frontend_contract::RouteErrorPolicy::Default, authentication = admin_permission_requirement(crate::domain_types::AdminPermission::SystemSettingsUpdate), method = frontend_contract::RouteMethod::Patch, mutation = frontend_contract::RouteMutation::Mutating, obligations = frontend_contract::AUTHENTICATED_MUTATING_ROUTE_COVERAGE_OBLIGATIONS, openapi_operation_id = "update_settings", path = "/system_settings", request = crate::domain_types::AdminUpdateSettingsReq, request_body = frontend_contract::RouteRequestBody::Json, response = crate::domain_types::AdminNoBody, success_status = frontend_contract::SuccessStatus::Code204, transport = frontend_contract::AuthenticatedTransport)]
pub struct AdminUpdateSettingsRoute;
