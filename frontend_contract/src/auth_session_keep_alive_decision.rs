#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq,
)]
pub enum AuthSessionKeepAliveDecision {
    RefreshNow,
    SkipAlreadyRunning,
    SkipIntervalOverflow,
    SkipMissing,
    SkipNotDue {
        next: crate::auth_session_instant::AuthSessionInstant,
    },
}
