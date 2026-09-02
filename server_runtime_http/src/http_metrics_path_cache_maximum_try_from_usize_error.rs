#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    thiserror::Error,
)]
pub enum HttpMetricsPathCacheMaximumTryFromUsizeError {
    #[error("{message}", message = constants_str::HTTP_METRICS_PATH_CACHE_MAXIMUM_MUST_BE_GREATER_THAN_ZERO)]
    Zero,
}
