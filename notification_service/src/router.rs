// The owner module retains lint-sensitive semantics from the original implementation.
#![allow(clippy::single_call_fn)]

pub(crate) fn router(
    state: crate::domain_types::NotificationState,
    body_maximum_bytes: crate::domain_types::NotificationBodyMaximumBytes,
) -> crate::domain_types::AxumNotificationRouter {
    let common_routes = axum::Router::from(common_routes::adapters::common_routes(
        common_routes::domain_types::ArcCommonRoutesAppState::from(std::sync::Arc::new(
            state.clone(),
        )),
    ));
    crate::domain_types::AxumNotificationRouter::from(
        super::notification_route_registry::NotificationRouteRegistry::router()
            .merge(super::notification_api_route_registry::NotificationApiRouteRegistry::router())
            .layer(axum::extract::DefaultBodyLimit::max(
                body_maximum_bytes.get(),
            ))
            .with_state(state)
            .merge(common_routes),
    )
}
