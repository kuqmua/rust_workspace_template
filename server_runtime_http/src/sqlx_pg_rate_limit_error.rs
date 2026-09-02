#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    thiserror::Error,
    proc_macro_newtype::FromInner,
    proc_macro_newtype::IntoInnerFrom,
)]
#[error(transparent)]
pub struct SqlxPgRateLimitError(sqlx::Error);
