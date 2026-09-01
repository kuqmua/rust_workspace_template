#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, newtype::FromInner)]
pub(super) struct TokioMockNotificationSender(
    tokio::sync::mpsc::UnboundedSender<
        server_runtime_http::runtime_notification_message::RuntimeNotificationMessage,
    >,
);

impl TokioMockNotificationSender {
    pub(super) fn send(
        &self,
        message: server_runtime_http::runtime_notification_message::RuntimeNotificationMessage,
    ) -> Result<
        (),
        tokio::sync::mpsc::error::SendError<
            server_runtime_http::runtime_notification_message::RuntimeNotificationMessage,
        >,
    > {
        self.0.send(message)
    }
}
