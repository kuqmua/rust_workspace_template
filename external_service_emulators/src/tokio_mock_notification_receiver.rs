use tokio::sync::mpsc::UnboundedReceiver;

#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, newtype::FromInner)]
pub(super) struct TokioMockNotificationReceiver(
    UnboundedReceiver<
        server_runtime_http::runtime_notification_message::RuntimeNotificationMessage,
    >,
);

impl TokioMockNotificationReceiver {
    pub(super) async fn receive(
        &mut self,
    ) -> Option<server_runtime_http::runtime_notification_message::RuntimeNotificationMessage> {
        self.0.recv().await
    }
}
