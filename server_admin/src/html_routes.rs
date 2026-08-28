pub(crate) fn html_routes(
    state: crate::SharedAdminAuthSvcStateArc,
    swagger_enabled: crate::AdminHtmlSwaggerEnabled,
) -> crate::AxumAdminAuthRouter {
    let router = axum::Router::from(crate::admin_html_page_router())
        .merge(axum::Router::from(crate::admin_html_action_router()));
    let router = if swagger_enabled.0 {
        router.merge(axum::Router::from(crate::swagger_router()))
    } else {
        router
    };
    crate::AxumAdminAuthRouter(router.with_state(state))
}
