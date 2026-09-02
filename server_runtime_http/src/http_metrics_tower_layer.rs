#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Clone, Debug)]
pub(super) struct HttpMetricsTowerLayer {
    paths: crate::shared_http_metrics_path_cache_arc::SharedHttpMetricsPathCacheArc,
}

impl From<crate::shared_http_metrics_path_cache_arc::SharedHttpMetricsPathCacheArc>
    for HttpMetricsTowerLayer
{
    fn from(
        shared_http_metrics_path_cache_arc: crate::shared_http_metrics_path_cache_arc::SharedHttpMetricsPathCacheArc,
    ) -> Self {
        Self {
            paths: shared_http_metrics_path_cache_arc,
        }
    }
}

impl<Service> tower::Layer<Service> for HttpMetricsTowerLayer {
    type Service = crate::http_metrics_service::HttpMetricsService<Service>;

    fn layer(&self, service: Service) -> Self::Service {
        crate::http_metrics_service::HttpMetricsService::new(service, self.paths.clone())
    }
}
