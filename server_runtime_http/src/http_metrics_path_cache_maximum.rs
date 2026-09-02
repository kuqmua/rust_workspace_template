#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    proc_macro_newtype::FromInner,
)]
pub struct HttpMetricsPathCacheMaximum(std::num::NonZeroUsize);

impl HttpMetricsPathCacheMaximum {
    pub(crate) const fn get(self) -> usize {
        self.0.get()
    }
}

impl TryFrom<usize> for HttpMetricsPathCacheMaximum {
    type Error = crate::http_metrics_path_cache_maximum_try_from_usize_error::HttpMetricsPathCacheMaximumTryFromUsizeError;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        std::num::NonZeroUsize::new(value)
            .map(Self::from)
            .ok_or(crate::http_metrics_path_cache_maximum_try_from_usize_error::HttpMetricsPathCacheMaximumTryFromUsizeError::Zero)
    }
}
