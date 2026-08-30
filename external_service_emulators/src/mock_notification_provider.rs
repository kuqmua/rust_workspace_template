#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[must_use]
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug)]
pub struct MockNotificationProvider {
    pub(super) sender: crate::tokio_mock_notification_sender::TokioMockNotificationSender,
}

impl server_runtime_http::notification_sender::NotificationSender for MockNotificationProvider {
    type Error = crate::mock_notification_provider_closed::MockNotificationProviderClosed;

    fn send(
        &self,
        message: server_runtime_http::notification_message::NotificationMessage,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send {
        std::future::ready(self.sender.0.send(message).map_err(|_error| {
            crate::mock_notification_provider_closed::MockNotificationProviderClosed::Closed
        }))
    }
}
