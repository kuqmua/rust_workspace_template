use super::{AdminHtmlSwaggerEnabled, AxumAdminAuthRouter, SharedAdminAuthSvcStateArc, html};

#[must_use]
pub fn admin_auth_html_routes(state: SharedAdminAuthSvcStateArc) -> AxumAdminAuthRouter {
    html::html_routes::html_routes(state, AdminHtmlSwaggerEnabled::from(true))
}
