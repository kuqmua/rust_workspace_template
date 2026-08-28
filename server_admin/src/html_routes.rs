pub(in super::super) fn html_routes(
    state: super::super::SharedAdminAuthSvcStateArc,
    swagger_enabled: super::super::AdminHtmlSwaggerEnabled,
) -> super::super::AxumAdminAuthRouter {
    let router = axum::Router::from(super::pages::admin_html_page_router()).merge(
        axum::Router::from(super::actions::admin_html_action_router()),
    );
    let router = if swagger_enabled.0 {
        router.merge(axum::Router::from(super::pages::swagger_router()))
    } else {
        router
    };
    super::super::AxumAdminAuthRouter(router.with_state(state))
}
