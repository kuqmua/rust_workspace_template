// The owner module retains lint-sensitive semantics from the original implementation.
#[allow(clippy::single_call_fn)] // Axum route endpoint is registered once by the route inventory
#[frontend_contract::domain_types::route_openapi(
    delegate = super::audit_export_log::audit_export_log,
    params(super::AdminAuditQuery),
    tag = "admin_audit"
)]
pub(super) async fn api_export_audit_log(
    auth: super::AdminAuthReq,
    query: super::AxumAdminQuery<super::AdminAuditQuery>,
) -> Result<super::AxumAdminResponse, super::AdminAuditExportError> {
}
