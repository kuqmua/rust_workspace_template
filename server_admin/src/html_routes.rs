pub(in super::super) fn routes(
    state: super::super::SharedAdminAuthSvcStateArc,
    swagger_enabled: super::super::AdminHtmlSwaggerEnabled,
) -> super::super::AxumAdminAuthRouter {
    let router = axum::Router::from(super::pages::router())
        .merge(axum::Router::from(super::actions::router()));
    let router = if swagger_enabled.0 {
        router.merge(axum::Router::from(super::pages::swagger_router()))
    } else {
        router
    };
    super::super::AxumAdminAuthRouter(router.with_state(state))
}
