use super::AdminHtmlSwaggerRouteRegistry;

pub(in crate::domain_types::auth::html) fn swagger_router()
-> super::super::super::AxumAdminStateRouter {
    super::super::super::AxumAdminStateRouter::from(AdminHtmlSwaggerRouteRegistry::router())
}
