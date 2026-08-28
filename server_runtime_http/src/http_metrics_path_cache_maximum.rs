#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq)]
pub struct HttpMetricsPathCacheMaximum(pub(super) super::HttpMetricsPathCacheMaximumNonZeroUsize);

impl TryFrom<usize> for HttpMetricsPathCacheMaximum {
    type Error = super::HttpMetricsPathCacheMaximumTryFromUsizeError;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        std::num::NonZeroUsize::new(value)
            .map(Self::from)
            .ok_or(super::HttpMetricsPathCacheMaximumTryFromUsizeError)
    }
}

impl From<std::num::NonZeroUsize> for HttpMetricsPathCacheMaximum {
    fn from(value: std::num::NonZeroUsize) -> Self {
        Self(super::HttpMetricsPathCacheMaximumNonZeroUsize::from(value))
    }
}
