#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    proc_macro_newtype_from_inner::FromInner,
    proc_macro_newtype_get_inner::GetInner,
)]
#[accessor(pub(crate))]
pub struct NotificationApiTokenRef<'value_lt>(&'value_lt str);

impl std::fmt::Debug for NotificationApiTokenRef<'_> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(constants_str::NOTIFICATION_API_TOKEN_REDACTED)
    }
}
