pub(in crate::domain_types::auth::html) fn router() -> super::super::super::AxumAdminStateRouter {
    super::super::super::AxumAdminStateRouter::from(
        super::AdminHtmlActionRouteRegistry::registry_router()
            .merge(axum::Router::from(super::auth::router()))
            .merge(axum::Router::from(super::roles::router()))
            .merge(axum::Router::from(super::sessions::router()))
            .merge(axum::Router::from(super::settings::router()))
            .merge(axum::Router::from(super::users::router())),
    )
}
