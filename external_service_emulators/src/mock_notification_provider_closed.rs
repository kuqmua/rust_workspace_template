#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error,
)]
#[error("{}", constants_str::test_fixtures::MOCK_NOTIFICATION_PROVIDER_CLOSED)]
pub struct MockNotificationProviderClosed;
