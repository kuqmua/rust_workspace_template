#[must_use]
#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Clone, Debug)]
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
        runtime_notification_message: server_runtime_http::runtime_notification_message::RuntimeNotificationMessage,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send {
        std::future::ready(
            self.sender
                .send(runtime_notification_message)
                .map_err(|_error| {
                    crate::mock_notification_provider_closed::MockNotificationProviderClosed::Closed
                }),
        )
    }
}
