#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug)]
pub struct NotificationServiceState<Sender> {
    pub(super) permits: crate::domain_types::ArcTokioSemaphore,
    pub(super) sender: Sender,
    pub(super) token: super::NotificationApiToken,
}

impl<Sender> NotificationServiceState<Sender> {
    #[must_use]
    pub fn new(
        token: super::NotificationApiToken,
        sender: Sender,
        maximum_concurrency: crate::domain_types::SemaphorePermitCountNonZeroUsize,
    ) -> Self {
        Self {
            permits: crate::domain_types::ArcTokioSemaphore::new(maximum_concurrency),
            sender,
            token,
        }
    }
}
