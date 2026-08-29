// The owner module retains lint-sensitive semantics from the original implementation.

#[frontend_contract_macros::route_openapi(
    delegate = crate::audit_export_log::audit_export_log,
    params(crate::admin_audit_query::AdminAuditQuery),
    tag = "admin_audit"
)]
pub(crate) async fn api_export_audit_log(
    auth: crate::admin_auth_req::AdminAuthReq,
    query: crate::axum_admin_query::AxumAdminQuery<crate::admin_audit_query::AdminAuditQuery>,
) -> Result<
    crate::axum_admin_response::AxumAdminResponse,
    crate::application_auth::AdminAuditExportError,
> {
}
