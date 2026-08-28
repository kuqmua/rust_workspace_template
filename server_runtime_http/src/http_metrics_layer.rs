#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug)]
pub struct HttpMetricsLayer {
    paths: super::SharedHttpMetricsPathCacheArc,
}

impl Default for HttpMetricsLayer {
    fn default() -> Self {
        Self::new(super::HttpMetricsPathCacheMaximum::from(
            std::num::NonZeroUsize::MIN.saturating_add(constants_usize::VALUE_4_096 - 1usize),
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
            paths: super::SharedHttpMetricsPathCacheArc::from(super::HttpMetricsPathCache {
                entries: super::HttpMetricsPathEntriesRwLock::from(std::sync::RwLock::new(
                    std::collections::HashMap::with_capacity(
                        path_cache_maximum.0.get().min(constants_usize::VALUE_4_096),
                    ),
                )),
                maximum: path_cache_maximum,
                unmatched: super::MetricsSharedString::from(metrics::SharedString::const_str(
                    constants_str::HTTP_METRICS_UNMATCHED_PATH,
                )),
            }),
        }
    }
}
