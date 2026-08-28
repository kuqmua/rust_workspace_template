use super::admin_permission_requirement;

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    frontend_contract::domain_types::TypedRoute,
)]
#[typed_route(error_policy = frontend_contract::domain_types::RouteErrorPolicy::ValidatedRead, authentication = admin_permission_requirement(crate::domain_types::AdminPermission::AuditLogExport), method = frontend_contract::domain_types::RouteMethod::Get, mutation = frontend_contract::domain_types::RouteMutation::ReadOnly, obligations = frontend_contract::domain_types::AUTHENTICATED_READ_ROUTE_COVERAGE_OBLIGATIONS, openapi_operation_id = "export_audit_log", path = "/audit_log/export", request = crate::domain_types::AdminNoBody, response = crate::domain_types::AdminAuditExport, success_status = frontend_contract::domain_types::SuccessStatus::Code200, transport = frontend_contract::domain_types::AuthenticatedTransport)]
pub struct AdminAuditExportRoute;
