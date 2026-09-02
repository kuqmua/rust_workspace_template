// The owner module retains lint-sensitive semantics from the original implementation.

#[proc_macro_frontend_contract::route_openapi(
    delegate = crate::audit_export_log::audit_export_log,
    params(crate::admin_audit_query::AdminAuditQuery),
    tag = "admin_audit"
)]
pub(crate) async fn api_export_audit_log(
    admin_auth_request: crate::admin_auth_request::AdminAuthRequest,
    axum_admin_query: crate::axum_admin_query::AxumAdminQuery<
        crate::admin_audit_query::AdminAuditQuery,
    >,
) -> Result<
    crate::axum_admin_response::AxumAdminResponse,
    crate::application_auth::AdminAuditExportError,
> {
}
