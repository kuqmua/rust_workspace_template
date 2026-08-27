#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, newtype::FromInner)]
pub struct NotificationApiTokenRef<'value_lt>(pub(super) &'value_lt str);

impl std::fmt::Debug for NotificationApiTokenRef<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(constants_str::NOTIFICATION_API_TOKEN_REDACTED)
    }
}
