// The owner module retains lint-sensitive semantics from the original implementation.

#[allow(clippy::single_call_fn)] // service startup owns the assembled notification router
pub(crate) fn build_notification_router(
    state: crate::notification_state::NotificationState,
    body_maximum_bytes: crate::notification_body_maximum_bytes::NotificationBodyMaximumBytes,
) -> crate::axum_notification_router::AxumNotificationRouter {
    let common_routes = axum::Router::from(common_routes::common_routes::common_routes(
        crate::shared_notification_state_arc::SharedNotificationStateArc::from_state(state.clone())
            .into_common_routes_app_state(),
    ));
    crate::axum_notification_router::AxumNotificationRouter::from(
        super::notification_route_registry::NotificationRouteRegistry::router()
            .merge(super::notification_api_route_registry::NotificationApiRouteRegistry::router())
            .layer(axum::extract::DefaultBodyLimit::max(
                body_maximum_bytes.get(),
            ))
            .with_state(state)
            .merge(common_routes),
    )
}
