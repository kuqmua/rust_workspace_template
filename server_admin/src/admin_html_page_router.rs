use crate::AdminHtmlPageRouteRegistry;

pub(crate) fn admin_html_page_router() -> crate::AxumAdminStateRouter {
    crate::AxumAdminStateRouter::from(AdminHtmlPageRouteRegistry::router())
}
