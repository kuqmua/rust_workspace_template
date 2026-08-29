#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug)]
pub struct NotificationServiceState<Sender> {
    pub(super) permits: crate::arc_tokio_semaphore::ArcTokioSemaphore,
    pub(super) sender: Sender,
    pub(super) token: crate::notification_api_token::NotificationApiToken,
}

impl<Sender> NotificationServiceState<Sender> {
    #[must_use]
    pub fn new(
        token: crate::notification_api_token::NotificationApiToken,
        sender: Sender,
        maximum_concurrency: crate::semaphore_permit_count_non_zero_usize::SemaphorePermitCountNonZeroUsize,
    ) -> Self {
        Self {
            permits: crate::arc_tokio_semaphore::ArcTokioSemaphore::new(maximum_concurrency),
            sender,
            token,
        }
    }
}
