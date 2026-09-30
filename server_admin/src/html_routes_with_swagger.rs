#[must_use]
pub fn html_routes_with_swagger(
    shared_admin_auth_service_state_arc: crate::shared_admin_auth_service_state_arc::SharedAdminAuthServiceStateArc,
    admin_html_swagger_enabled: crate::admin_html_swagger_enabled::AdminHtmlSwaggerEnabled,
) -> crate::axum_admin_auth_router::AxumAdminAuthRouter {
    crate::html_routes::html_routes(
        shared_admin_auth_service_state_arc,
        admin_html_swagger_enabled,
    )
}
