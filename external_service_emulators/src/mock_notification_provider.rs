#![allow(
    clippy::arbitrary_source_item_ordering,
    clippy::module_inception,
    reason = "the flat source facade keeps its owner adjacent to implementation while declaring sibling modules"
)]
#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[must_use]
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
