#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    thiserror::Error,
)]
pub enum ContentSecurityPolicyError {
    #[error("content security policy must not be empty")]
    Empty,
    #[error("content security policy is too long or contains forbidden line breaks")]
    Invalid,
}
