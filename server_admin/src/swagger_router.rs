use crate::AdminHtmlSwaggerRouteRegistry;

#[allow(clippy::single_call_fn)] // named route or composition boundary has one registry or orchestration owner
pub(crate) fn swagger_router() -> crate::AxumAdminStateRouter {
    crate::AxumAdminStateRouter::from(AdminHtmlSwaggerRouteRegistry::router())
}
