#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug)]
pub struct HttpMetricsLayer {
    paths: crate::shared_http_metrics_path_cache_arc::SharedHttpMetricsPathCacheArc,
}

impl Default for HttpMetricsLayer {
    fn default() -> Self {
        Self::new(
            crate::http_metrics_path_cache_maximum::HttpMetricsPathCacheMaximum::from(
                std::num::NonZeroUsize::MIN.saturating_add(constants_usize::VALUE_4_096 - 1usize),
            ),
        )
    }
}

impl HttpMetricsLayer {
    #[must_use]
    pub fn apply(self, router: crate::axum_router::AxumRouter) -> crate::axum_router::AxumRouter {
        crate::axum_router::AxumRouter::from(
            axum::Router::from(router).layer(
                crate::http_metrics_tower_layer::HttpMetricsTowerLayer { paths: self.paths },
            ),
        )
    }

    #[must_use]
    pub fn new(
        path_cache_maximum: crate::http_metrics_path_cache_maximum::HttpMetricsPathCacheMaximum,
    ) -> Self {
        Self {
            paths: crate::shared_http_metrics_path_cache_arc::SharedHttpMetricsPathCacheArc::from(
                crate::http_metrics_path_cache::HttpMetricsPathCache {
                    entries:
                        crate::http_metrics_path_entries_rw_lock::HttpMetricsPathEntriesRwLock::from(
                            std::sync::RwLock::new(std::collections::HashMap::with_capacity(
                                path_cache_maximum.0.get().min(constants_usize::VALUE_4_096),
                            )),
                        ),
                    maximum: path_cache_maximum,
                    unmatched: crate::metrics_shared_string::MetricsSharedString::from(
                        metrics::SharedString::const_str(
                            constants_str::catalog::HTTP_METRICS_UNMATCHED_PATH,
                        ),
                    ),
                },
            ),
        }
    }
}
