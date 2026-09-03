#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    proc_macro_newtype_from_inner::FromInner,
)]
pub(super) struct RequestTimeoutTowerLayer(crate::request_timeout_duration::RequestTimeoutDuration);

impl<Service> tower::Layer<Service> for RequestTimeoutTowerLayer {
    type Service = crate::request_timeout_service::RequestTimeoutService<Service>;

    fn layer(&self, service: Service) -> Self::Service {
        crate::request_timeout_service::RequestTimeoutService::new(service, self.0)
    }
}
