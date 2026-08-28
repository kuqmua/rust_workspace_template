use super::admin_permission_requirement;

#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, frontend_contract::TypedRoute,
)]
#[typed_route(error_policy = frontend_contract::RouteErrorPolicy::ValidatedRead, authentication = admin_permission_requirement(crate::domain_types::AdminPermission::AuditLogRead), method = frontend_contract::RouteMethod::Get, mutation = frontend_contract::RouteMutation::ReadOnly, obligations = frontend_contract::AUTHENTICATED_READ_ROUTE_COVERAGE_OBLIGATIONS, openapi_operation_id = "audit_log", path = "/audit_log", request = crate::domain_types::AdminNoBody, response = crate::domain_types::AdminAuditPage, success_status = frontend_contract::SuccessStatus::Code200, transport = frontend_contract::AuthenticatedTransport)]
pub struct AdminAuditLogRoute;
