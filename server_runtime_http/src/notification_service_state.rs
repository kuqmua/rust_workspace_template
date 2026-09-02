#[derive(proc_macro_getters::Getters)]
#[getters(bare)]
#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Clone, Debug)]
pub struct NotificationServiceState<Sender> {
    permits: crate::arc_tokio_semaphore::ArcTokioSemaphore,
    sender: Sender,
    token: crate::notification_api_token::NotificationApiToken,
}

impl<Sender> NotificationServiceState<Sender> {
    #[must_use]
    pub fn new(
        notification_api_token: crate::notification_api_token::NotificationApiToken,
        sender: Sender,
        semaphore_permit_count_non_zero_usize: crate::semaphore_permit_count_non_zero_usize::SemaphorePermitCountNonZeroUsize,
    ) -> Self {
        Self {
            permits: crate::arc_tokio_semaphore::ArcTokioSemaphore::new(
                semaphore_permit_count_non_zero_usize,
            ),
            sender,
            token: notification_api_token,
        }
    }
}
