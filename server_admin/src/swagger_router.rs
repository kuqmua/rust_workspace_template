use crate::AdminHtmlSwaggerRouteRegistry;

pub(crate) fn swagger_router() -> crate::AxumAdminStateRouter {
    crate::AxumAdminStateRouter::from(AdminHtmlSwaggerRouteRegistry::router())
}
