#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    newtype::FromInner,
)]
pub struct HttpMetricsPathCacheMaximum(pub(super) std::num::NonZeroUsize);

impl TryFrom<usize> for HttpMetricsPathCacheMaximum {
    type Error = crate::http_metrics_path_cache_maximum_try_from_usize_error::HttpMetricsPathCacheMaximumTryFromUsizeError;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        std::num::NonZeroUsize::new(value)
            .map(Self::from)
            .ok_or(crate::http_metrics_path_cache_maximum_try_from_usize_error::HttpMetricsPathCacheMaximumTryFromUsizeError::Zero)
    }
}
