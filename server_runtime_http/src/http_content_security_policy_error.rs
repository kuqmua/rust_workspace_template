#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, thiserror::Error,
)]
pub enum HttpContentSecurityPolicyError {
    #[error("content security policy is not a valid HTTP header value")]
    InvalidHeaderValue,
}
