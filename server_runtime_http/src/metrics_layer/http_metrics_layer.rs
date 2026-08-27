#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug)]
pub struct HttpMetricsLayer {
    paths: super::SharedHttpMetricsPathCacheArc,
}

impl Default for HttpMetricsLayer {
    fn default() -> Self {
        Self::new(super::HttpMetricsPathCacheMaximum::from(
            std::num::NonZeroUsize::MIN
                .saturating_add(super::DEFAULT_HTTP_METRICS_PATH_CACHE_MAXIMUM - 1usize),
        ))
    }
}

impl HttpMetricsLayer {
    #[must_use]
    pub fn apply(self, router: crate::domain_types::AxumRouter) -> crate::domain_types::AxumRouter {
        crate::domain_types::AxumRouter::from(
            axum::Router::from(router).layer(super::HttpMetricsTowerLayer { paths: self.paths }),
        )
    }

    #[must_use]
    pub fn new(path_cache_maximum: super::HttpMetricsPathCacheMaximum) -> Self {
        Self {
            paths: super::SharedHttpMetricsPathCacheArc::from(super::HttpMetricsPathCache::new(
                path_cache_maximum,
            )),
        }
    }
}
