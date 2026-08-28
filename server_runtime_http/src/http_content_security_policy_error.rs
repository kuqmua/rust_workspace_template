#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, thiserror::Error)]
#[error("content security policy is not a valid HTTP header value")]
pub struct HttpContentSecurityPolicyError;
