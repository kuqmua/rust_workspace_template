pub fn notification_router<Sender>(
    state: crate::notification_service_state::NotificationServiceState<Sender>,
) -> crate::axum_notification_router::AxumNotificationRouter
where
    Sender: crate::notification_sender::NotificationSender,
{
    crate::axum_notification_router::AxumNotificationRouter::from(
        axum::Router::new()
            .route(
                constants_str::NOTIFICATIONS_PATH,
                axum::routing::post(crate::send_notification::send_notification::<Sender>),
            )
            .with_state(state),
    )
}
