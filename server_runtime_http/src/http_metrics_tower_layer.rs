#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug)]
pub(super) struct HttpMetricsTowerLayer {
    pub(super) paths: crate::shared_http_metrics_path_cache_arc::SharedHttpMetricsPathCacheArc,
}

impl<Service> tower::Layer<Service> for HttpMetricsTowerLayer {
    type Service = crate::http_metrics_service::HttpMetricsService<Service>;

    fn layer(&self, inner: Service) -> Self::Service {
        crate::http_metrics_service::HttpMetricsService {
            inner,
            paths: self.paths.clone(),
        }
    }
}
