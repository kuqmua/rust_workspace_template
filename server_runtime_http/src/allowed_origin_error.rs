#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error,
)]
pub enum AllowedOriginError {
    #[error("{message}", message = constants_str::catalog::ALLOWED_HTTP_ORIGIN_IS_INVALID)]
    Invalid,
}
