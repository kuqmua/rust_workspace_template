use crate::AdminHtmlPageRouteRegistry;

#[allow(clippy::single_call_fn)] // named route or composition boundary has one registry or orchestration owner
pub(crate) fn admin_html_page_router() -> crate::AxumAdminStateRouter {
    crate::AxumAdminStateRouter::from(AdminHtmlPageRouteRegistry::router())
}
