// The owner module retains lint-sensitive semantics from the original implementation.

#[allow(clippy::single_call_fn)] // service startup owns the assembled notification router
pub(crate) fn build_notification_router(
    state: crate::NotificationState,
    body_maximum_bytes: crate::NotificationBodyMaximumBytes,
) -> crate::AxumNotificationRouter {
    let common_routes = axum::Router::from(common_routes::common_routes(
        common_routes::ArcCommonRoutesAppState::from(std::sync::Arc::new(state.clone())),
    ));
    crate::AxumNotificationRouter::from(
        super::notification_route_registry::NotificationRouteRegistry::router()
            .merge(super::notification_api_route_registry::NotificationApiRouteRegistry::router())
            .layer(axum::extract::DefaultBodyLimit::max(
                body_maximum_bytes.get(),
            ))
            .with_state(state)
            .merge(common_routes),
    )
}
