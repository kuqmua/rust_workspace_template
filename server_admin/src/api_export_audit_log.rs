// The owner module retains lint-sensitive semantics from the original implementation.

#[frontend_contract::domain_types::route_openapi(
    delegate = crate::audit_export_log::audit_export_log,
    params(crate::AdminAuditQuery),
    tag = "admin_audit"
)]
pub(crate) async fn api_export_audit_log(
    auth: crate::AdminAuthReq,
    query: crate::AxumAdminQuery<crate::AdminAuditQuery>,
) -> Result<crate::AxumAdminResponse, crate::AdminAuditExportError> {
}
