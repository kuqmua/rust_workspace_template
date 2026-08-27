#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug)]
pub(super) struct HttpMetricsTowerLayer {
    pub(super) paths: super::SharedHttpMetricsPathCacheArc,
}

impl<Service> tower::Layer<Service> for HttpMetricsTowerLayer {
    type Service = super::HttpMetricsService<Service>;

    fn layer(&self, inner: Service) -> Self::Service {
        super::HttpMetricsService {
            inner,
            paths: self.paths.clone(),
        }
    }
}
