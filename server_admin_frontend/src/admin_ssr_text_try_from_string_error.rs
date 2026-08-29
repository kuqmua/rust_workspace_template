#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error,
)]
#[error("{message}", message = constants_str::test_fixtures::ADMIN_SSR_TITLE_TOO_LONG)]
pub struct AdminSsrTextTryFromStringError;
