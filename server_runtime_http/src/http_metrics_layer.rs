#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Clone, Debug)]
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
    pub fn apply(
        self,
        axum_router: crate::axum_router::AxumRouter,
    ) -> crate::axum_router::AxumRouter {
        crate::axum_router::AxumRouter::from(
            axum::Router::from(axum_router)
                .layer(crate::http_metrics_tower_layer::HttpMetricsTowerLayer::from(self.paths)),
        )
    }

    #[must_use]
    pub fn new(
        http_metrics_path_cache_maximum: crate::http_metrics_path_cache_maximum::HttpMetricsPathCacheMaximum,
    ) -> Self {
        Self {
            paths: crate::shared_http_metrics_path_cache_arc::SharedHttpMetricsPathCacheArc::from(
                crate::http_metrics_path_cache::HttpMetricsPathCache::from(
                    http_metrics_path_cache_maximum,
                ),
            ),
        }
    }
}
