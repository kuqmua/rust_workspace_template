#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    thiserror::Error,
)]
pub enum OutboundUrlError {
    #[error("outbound URL contains a forbidden control character")]
    ControlCharacter,
    #[error("outbound URL resolves to a forbidden address")]
    ForbiddenHost,
    #[error("outbound URL is invalid")]
    Invalid,
    #[error("outbound URL has no host")]
    MissingHost,
    #[error("outbound URL did not resolve to an address")]
    MissingResolvedAddress,
    #[error("outbound URL scheme is not allowed")]
    Scheme,
    #[error("outbound URL must not contain user information")]
    UserInfo,
}
