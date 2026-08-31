#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, newtype::DerefInner)]
pub(super) struct SharedHttpMetricsPathCacheArc(
    std::sync::Arc<crate::http_metrics_path_cache::HttpMetricsPathCache>,
);

impl From<crate::http_metrics_path_cache::HttpMetricsPathCache> for SharedHttpMetricsPathCacheArc {
    fn from(value: crate::http_metrics_path_cache::HttpMetricsPathCache) -> Self {
        Self(std::sync::Arc::from(value))
    }
}
