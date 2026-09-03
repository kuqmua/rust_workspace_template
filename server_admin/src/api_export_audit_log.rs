#[proc_macro_frontend_contract::route_openapi(
    params(crate::admin_audit_query::AdminAuditQuery),
    tag = "admin_audit"
)]
#[allow(
    clippy::single_call_fn,
    reason = "typed route registration requires a named endpoint function"
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
    crate::audit_export_log::audit_export_log(admin_auth_request, axum_admin_query)
        .await
        .map_err(crate::application_auth::AdminAuditExportError::from)
}
