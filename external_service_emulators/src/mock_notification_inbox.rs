#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug)]
pub struct MockNotificationInbox {
    receiver: crate::tokio_mock_notification_receiver::TokioMockNotificationReceiver,
}

impl From<crate::tokio_mock_notification_receiver::TokioMockNotificationReceiver>
    for MockNotificationInbox
{
    fn from(
        receiver: crate::tokio_mock_notification_receiver::TokioMockNotificationReceiver,
    ) -> Self {
        Self { receiver }
    }
}

impl MockNotificationInbox {
    pub async fn receive(
        &mut self,
    ) -> Option<server_runtime_http::runtime_notification_message::RuntimeNotificationMessage> {
        self.receiver.receive().await
    }
}
