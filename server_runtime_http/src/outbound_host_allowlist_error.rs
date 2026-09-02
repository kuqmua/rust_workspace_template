#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    thiserror::Error,
)]
pub enum OutboundHostAllowlistError {
    #[error("outbound host allowlist must not be empty")]
    Empty,
    #[error("outbound host is not present in the allowlist")]
    HostNotAllowed,
    #[error("outbound allowlist host is invalid")]
    InvalidHost,
    #[error("outbound host allowlist exceeds 64 entries")]
    TooManyHosts,
}
