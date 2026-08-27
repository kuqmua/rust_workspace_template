#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug)]
pub struct MockNotificationInbox {
    pub(super) receiver: super::TokioMockNotificationReceiver,
}

impl MockNotificationInbox {
    pub async fn receive(
        &mut self,
    ) -> Option<server_runtime_http::domain_types::NotificationMessage> {
        self.receiver.0.recv().await
    }
}
