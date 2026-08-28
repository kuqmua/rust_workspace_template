#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error,
)]
#[error("{message}", message = constants_str::ALLOWED_HTTP_ORIGIN_IS_INVALID)]
pub struct AllowedOriginError;
