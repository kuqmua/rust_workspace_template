#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    proc_macro_newtype::FromInner,
)]
pub struct NotificationApiTokenRef<'value_lt>(&'value_lt str);

impl<'value_lt> NotificationApiTokenRef<'value_lt> {
    pub(crate) const fn get(self) -> &'value_lt str {
        self.0
    }
}

impl std::fmt::Debug for NotificationApiTokenRef<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(constants_str::NOTIFICATION_API_TOKEN_REDACTED)
    }
}
