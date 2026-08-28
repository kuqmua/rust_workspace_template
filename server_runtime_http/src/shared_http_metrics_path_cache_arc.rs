#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug)]
pub(super) struct SharedHttpMetricsPathCacheArc(
    pub(super) std::sync::Arc<super::HttpMetricsPathCache>,
);

impl From<super::HttpMetricsPathCache> for SharedHttpMetricsPathCacheArc {
    fn from(value: super::HttpMetricsPathCache) -> Self {
        Self(std::sync::Arc::from(value))
    }
}
