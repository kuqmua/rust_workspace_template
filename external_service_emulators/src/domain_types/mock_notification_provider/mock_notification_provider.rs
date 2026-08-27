#[must_use]
pub fn mock_notification_provider() -> (
    super::super::MockNotificationProvider,
    super::super::MockNotificationInbox,
) {
    let (sender, receiver) = tokio::sync::mpsc::unbounded_channel();
    (
        super::super::MockNotificationProvider {
            sender: super::super::TokioMockNotificationSender::from(sender),
        },
        super::super::MockNotificationInbox {
            receiver: super::super::TokioMockNotificationReceiver::from(receiver),
        },
    )
}
