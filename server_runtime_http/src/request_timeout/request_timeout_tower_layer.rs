#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, newtype::FromInner)]
pub(super) struct RequestTimeoutTowerLayer(pub(super) super::super::RequestTimeoutDuration);

impl<Service> tower::Layer<Service> for RequestTimeoutTowerLayer {
    type Service = super::RequestTimeoutService<Service>;

    fn layer(&self, inner: Service) -> Self::Service {
        super::RequestTimeoutService {
            inner,
            timeout: self.0,
        }
    }
}
