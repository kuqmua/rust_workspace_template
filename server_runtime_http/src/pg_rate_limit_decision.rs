#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq,
)]
pub enum PgRateLimitDecision {
    Allowed,
    Limited(crate::pg_rate_limit_window_seconds::PgRateLimitWindowSeconds),
}
