#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error,
)]
#[error("administrator SSR HTML exceeds the size limit")]
pub struct AdminSsrHtmlTryFromStringError;
