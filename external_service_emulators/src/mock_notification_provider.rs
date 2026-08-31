#[must_use]
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug)]
pub struct MockNotificationProvider {
    sender: crate::tokio_mock_notification_sender::TokioMockNotificationSender,
}

impl From<crate::tokio_mock_notification_sender::TokioMockNotificationSender>
    for MockNotificationProvider
{
    fn from(value: crate::tokio_mock_notification_sender::TokioMockNotificationSender) -> Self {
        Self { sender: value }
    }
}

impl server_runtime_http::notification_sender::NotificationSender for MockNotificationProvider {
    type Error = crate::mock_notification_provider_closed::MockNotificationProviderClosed;

    fn send(
        &self,
        message: server_runtime_http::runtime_notification_message::RuntimeNotificationMessage,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send {
        std::future::ready(self.sender.send(message).map_err(|_error| {
            crate::mock_notification_provider_closed::MockNotificationProviderClosed::Closed
        }))
    }
}
