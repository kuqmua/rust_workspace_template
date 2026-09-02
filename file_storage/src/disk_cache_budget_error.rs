#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    thiserror::Error,
)]
pub enum DiskCacheBudgetError {
    #[error("incoming cache entry exceeds the cache budget")]
    IncomingTooLarge,
    #[error("cache size calculation overflowed")]
    SizeOverflow,
}
