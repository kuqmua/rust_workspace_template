#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
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
