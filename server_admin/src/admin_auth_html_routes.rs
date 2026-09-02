#[must_use]
pub fn admin_auth_html_routes(
    shared_admin_auth_svc_state_arc: crate::shared_admin_auth_svc_state_arc::SharedAdminAuthSvcStateArc,
) -> crate::axum_admin_auth_router::AxumAdminAuthRouter {
    crate::html_routes::html_routes(
        shared_admin_auth_svc_state_arc,
        crate::admin_html_swagger_enabled::AdminHtmlSwaggerEnabled::from(true),
    )
}
