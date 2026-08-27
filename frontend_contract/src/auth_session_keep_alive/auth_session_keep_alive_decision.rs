#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq)]
pub enum AuthSessionKeepAliveDecision {
    RefreshNow,
    SkipAlreadyRunning,
    SkipMissing,
    SkipNotDue { next: super::AuthSessionInstant },
}
