#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the public request-id layer constructs this private tower layer owner"
)]
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug)]
pub(super) struct RequestIdTowerLayer {
    pub(super) span_config: Option<super::http_request_span_config::HttpRequestSpanConfig>,
}
impl<Service> tower::Layer<Service> for RequestIdTowerLayer {
    type Service = super::request_id_service::RequestIdService<Service>;
    fn layer(&self, inner: Service) -> Self::Service {
        super::request_id_service::RequestIdService {
            inner,
            span_config: self.span_config.clone(),
        }
    }
}
