#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Debug)]
pub struct MockNotificationInbox {
    receiver: crate::tokio_mock_notification_receiver::TokioMockNotificationReceiver,
}

impl From<crate::tokio_mock_notification_receiver::TokioMockNotificationReceiver>
    for MockNotificationInbox
{
    fn from(
        tokio_mock_notification_receiver: crate::tokio_mock_notification_receiver::TokioMockNotificationReceiver,
    ) -> Self {
        Self {
            receiver: tokio_mock_notification_receiver,
        }
    }
}

impl MockNotificationInbox {
    pub async fn receive(
        &mut self,
    ) -> Option<server_runtime_http::runtime_notification_message::RuntimeNotificationMessage> {
        self.receiver.receive().await
    }
}
