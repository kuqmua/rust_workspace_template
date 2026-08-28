use super::{
    MockNotificationInbox, MockNotificationProvider, TokioMockNotificationReceiver,
    TokioMockNotificationSender,
};

pub fn create_mock_notification_provider() -> (MockNotificationProvider, MockNotificationInbox) {
    let (sender, receiver) = tokio::sync::mpsc::unbounded_channel();
    (
        MockNotificationProvider {
            sender: TokioMockNotificationSender::from(sender),
        },
        MockNotificationInbox {
            receiver: TokioMockNotificationReceiver::from(receiver),
        },
    )
}
