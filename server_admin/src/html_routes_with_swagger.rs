#[must_use]
pub fn html_routes_with_swagger(
    state: crate::shared_admin_auth_svc_state_arc::SharedAdminAuthSvcStateArc,
    swagger_enabled: crate::admin_html_swagger_enabled::AdminHtmlSwaggerEnabled,
) -> crate::axum_admin_auth_router::AxumAdminAuthRouter {
    crate::html_routes::html_routes(state, swagger_enabled)
}
