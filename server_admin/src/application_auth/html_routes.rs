use super::{AdminHtmlSwaggerEnabled, AxumAdminAuthRouter, SharedAdminAuthSvcStateArc, html};

#[must_use]
pub fn html_routes(state: SharedAdminAuthSvcStateArc) -> AxumAdminAuthRouter {
    html::html_routes::html_routes(state, AdminHtmlSwaggerEnabled::from(true))
}
