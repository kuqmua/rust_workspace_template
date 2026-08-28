#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
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
