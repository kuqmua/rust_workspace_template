use super::AdminHtmlPageRouteRegistry;

pub(in crate::domain_types::auth::html) fn admin_html_page_router()
-> super::super::super::AxumAdminStateRouter {
    super::super::super::AxumAdminStateRouter::from(AdminHtmlPageRouteRegistry::router())
}
