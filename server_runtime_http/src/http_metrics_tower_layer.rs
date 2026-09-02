#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Clone, Debug)]
pub(super) struct HttpMetricsTowerLayer {
    paths: crate::shared_http_metrics_path_cache_arc::SharedHttpMetricsPathCacheArc,
}

impl From<crate::shared_http_metrics_path_cache_arc::SharedHttpMetricsPathCacheArc>
    for HttpMetricsTowerLayer
{
    fn from(
        value: crate::shared_http_metrics_path_cache_arc::SharedHttpMetricsPathCacheArc,
    ) -> Self {
        Self { paths: value }
    }
}

impl<Service> tower::Layer<Service> for HttpMetricsTowerLayer {
    type Service = crate::http_metrics_service::HttpMetricsService<Service>;

    fn layer(&self, inner: Service) -> Self::Service {
        crate::http_metrics_service::HttpMetricsService::new(inner, self.paths.clone())
    }
}
