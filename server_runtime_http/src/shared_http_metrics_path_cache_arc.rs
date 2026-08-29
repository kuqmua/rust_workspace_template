#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug)]
pub(super) struct SharedHttpMetricsPathCacheArc(
    pub(super) std::sync::Arc<crate::http_metrics_path_cache::HttpMetricsPathCache>,
);

impl From<crate::http_metrics_path_cache::HttpMetricsPathCache> for SharedHttpMetricsPathCacheArc {
    fn from(value: crate::http_metrics_path_cache::HttpMetricsPathCache) -> Self {
        Self(std::sync::Arc::from(value))
    }
}
