#[allow(
    clippy::single_call_fn,
    reason = "build notification router remains a named owner because its boundary role is clearer and directly testable"
)]
pub(crate) fn build_notification_router(
    notification_state: crate::notification_state::NotificationState,
    notification_body_maximum_bytes: crate::notification_body_maximum_bytes::NotificationBodyMaximumBytes,
) -> crate::notification_axum_router::NotificationAxumRouter {
    let common_routes = axum::Router::from(common_routes::common_routes::common_routes(
        crate::shared_notification_state_arc::SharedNotificationStateArc::from_state(
            notification_state.clone(),
        )
        .into_common_routes_app_state(),
    ));
    crate::notification_axum_router::NotificationAxumRouter::from(
        super::notification_route_registry::router()
            .merge(super::notification_api_route_registry::router())
            .layer(axum::extract::DefaultBodyLimit::max(
                notification_body_maximum_bytes.get(),
            ))
            .with_state(notification_state)
            .merge(common_routes),
    )
}
