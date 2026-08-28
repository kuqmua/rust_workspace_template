use crate::{AdminHtmlSwaggerEnabled, AxumAdminAuthRouter, SharedAdminAuthSvcStateArc, html};

#[must_use]
pub fn html_routes_with_swagger(
    state: SharedAdminAuthSvcStateArc,
    swagger_enabled: AdminHtmlSwaggerEnabled,
) -> AxumAdminAuthRouter {
    html::html_routes::html_routes(state, swagger_enabled)
}
