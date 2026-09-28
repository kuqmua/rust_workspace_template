#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    thiserror::Error,
)]
pub enum AuthSessionKeepAliveError {
    #[error("authentication session refresh interval must not be zero")]
    ZeroInterval,
    #[error("{}", constants_str::AUTH_SESSION_REFRESH_OVERFLOW)]
    IntervalOverflow,
}
