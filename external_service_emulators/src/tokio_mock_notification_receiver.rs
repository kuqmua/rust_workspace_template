#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    proc_macro_newtype_from_inner::FromInner,
)]
pub(super) struct TokioMockNotificationReceiver(
    tokio::sync::mpsc::UnboundedReceiver<
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
