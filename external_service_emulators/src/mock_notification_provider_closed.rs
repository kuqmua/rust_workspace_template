#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error,
)]
pub enum MockNotificationProviderClosed {
    #[error("{}", constants_str::MOCK_NOTIFICATION_PROVIDER_CLOSED)]
    Closed,
}
