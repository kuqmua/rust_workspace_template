#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    proc_macro_newtype::DerefInner,
)]
pub(super) struct SharedHttpMetricsPathCacheArc(
    std::sync::Arc<crate::http_metrics_path_cache::HttpMetricsPathCache>,
);

impl From<crate::http_metrics_path_cache::HttpMetricsPathCache> for SharedHttpMetricsPathCacheArc {
    fn from(http_metrics_path_cache: crate::http_metrics_path_cache::HttpMetricsPathCache) -> Self {
        Self(std::sync::Arc::from(http_metrics_path_cache))
    }
}
