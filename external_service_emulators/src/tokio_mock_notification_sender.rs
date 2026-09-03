#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    proc_macro_newtype_from_inner::FromInner,
)]
pub(super) struct TokioMockNotificationSender(
    tokio::sync::mpsc::UnboundedSender<
        server_runtime_http::runtime_notification_message::RuntimeNotificationMessage,
    >,
);

impl TokioMockNotificationSender {
    pub(super) fn send(
        &self,
        runtime_notification_message: server_runtime_http::runtime_notification_message::RuntimeNotificationMessage,
    ) -> Result<
        (),
        tokio::sync::mpsc::error::SendError<
            server_runtime_http::runtime_notification_message::RuntimeNotificationMessage,
        >,
    > {
        self.0.send(runtime_notification_message)
    }
}
