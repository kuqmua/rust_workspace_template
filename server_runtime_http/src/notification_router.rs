pub fn notification_router<Sender>(
    state: super::NotificationServiceState<Sender>,
) -> super::AxumNotificationRouter
where
    Sender: super::NotificationSender,
{
    super::AxumNotificationRouter::from(
        axum::Router::new()
            .route(
                constants_str::NOTIFICATIONS_PATH,
                axum::routing::post(super::send_notification::<Sender>),
            )
            .with_state(state),
    )
}
