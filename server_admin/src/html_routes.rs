pub(crate) fn html_routes(
    state: crate::SharedAdminAuthSvcStateArc,
    swagger_enabled: crate::AdminHtmlSwaggerEnabled,
) -> crate::AxumAdminAuthRouter {
    let router = crate::AdminHtmlPageRouteRegistry::router().merge(
        crate::AdminHtmlActionRouteRegistry::router()
            .merge(crate::AdminHtmlAuthActionRouteRegistry::router())
            .merge(crate::AdminHtmlRoleActionRouteRegistry::router())
            .merge(crate::AdminHtmlSessionActionRouteRegistry::router())
            .merge(crate::AdminHtmlSettingsActionRouteRegistry::router())
            .merge(crate::AdminHtmlUserActionRouteRegistry::router()),
    );
    let router = if swagger_enabled.0 {
        router.merge(crate::AdminHtmlSwaggerRouteRegistry::router())
    } else {
        router
    };
    crate::AxumAdminAuthRouter(router.with_state(state))
}
