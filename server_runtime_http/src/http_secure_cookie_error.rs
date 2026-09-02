#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    thiserror::Error,
)]
pub enum HttpSecureCookieError {
    #[error("generated Set-Cookie header is invalid")]
    InvalidHeader,
    #[error("invalid cookie name")]
    InvalidName,
    #[error("invalid cookie value")]
    InvalidValue,
}
