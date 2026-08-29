pub fn create_mock_notification_provider() -> (
    crate::mock_notification_provider::MockNotificationProvider,
    crate::mock_notification_inbox::MockNotificationInbox,
) {
    let (sender, receiver) = tokio::sync::mpsc::unbounded_channel();
    (
        crate::mock_notification_provider::MockNotificationProvider {
            sender: crate::tokio_mock_notification_sender::TokioMockNotificationSender::from(
                sender,
            ),
        },
        crate::mock_notification_inbox::MockNotificationInbox {
            receiver: crate::tokio_mock_notification_receiver::TokioMockNotificationReceiver::from(
                receiver,
            ),
        },
    )
}
