#[path = "mock_notification_provider/mock_notification_provider.rs"]
mod mock_notification_provider;

pub use mock_notification_provider::mock_notification_provider;

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug)]
pub struct MockNotificationProvider {
    pub(super) sender: super::TokioMockNotificationSender,
}

impl server_runtime_http::domain_types::NotificationSender for MockNotificationProvider {
    type Error = super::MockNotificationProviderClosed;

    fn send(
        &self,
        message: server_runtime_http::domain_types::NotificationMessage,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send {
        std::future::ready(
            self.sender
                .0
                .send(message)
                .map_err(|_error| super::MockNotificationProviderClosed),
        )
    }
}
