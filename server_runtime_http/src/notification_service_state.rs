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

#[cfg(test)]
mod tests {
    #[test]
    fn test_notification_state_clones_share_and_release_concurrency_permits() {
        let token = crate::notification_api_token::NotificationApiToken::try_from(
            constants_str::X.to_owned(),
        );
        assert!(token.is_ok_and(|notification_api_token| {
            let state = super::NotificationServiceState::new(
                notification_api_token.clone(),
                notification_api_token.clone(),
                crate::semaphore_permit_count_non_zero_usize::SemaphorePermitCountNonZeroUsize::from(
                    std::num::NonZeroUsize::MIN,
                ),
            );
            let cloned = state.clone();
            assert_eq!(state.token(), &notification_api_token);
            assert_eq!(cloned.sender(), &notification_api_token);
            let initial_permit = state.permits().try_acquire();
            assert!(initial_permit.is_some());
            assert!(cloned.permits().try_acquire().is_none());
            drop(initial_permit);
            let cloned_permit = cloned.permits().try_acquire();
            assert!(cloned_permit.is_some());
            assert!(state.permits().try_acquire().is_none());
            drop(cloned_permit);
            state.permits().try_acquire().is_some()
        }));
    }
}
