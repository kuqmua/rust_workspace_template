#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, proc_macro_new::New,
)]
#[constructor(pub(crate))]
pub(super) struct RequestIdTowerLayer {
    span_config: Option<super::http_request_span_config::HttpRequestSpanConfig>,
}
impl<Service> tower::Layer<Service> for RequestIdTowerLayer {
    type Service = super::request_id_service::RequestIdService<Service>;
    fn layer(&self, inner: Service) -> Self::Service {
        super::request_id_service::RequestIdService::new(inner, self.span_config.clone())
    }
}
