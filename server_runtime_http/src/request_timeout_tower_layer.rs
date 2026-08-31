#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, newtype::FromInner)]
pub(super) struct RequestTimeoutTowerLayer(crate::request_timeout_duration::RequestTimeoutDuration);

impl<Service> tower::Layer<Service> for RequestTimeoutTowerLayer {
    type Service = crate::request_timeout_service::RequestTimeoutService<Service>;

    fn layer(&self, inner: Service) -> Self::Service {
        crate::request_timeout_service::RequestTimeoutService::new(inner, self.0)
    }
}
