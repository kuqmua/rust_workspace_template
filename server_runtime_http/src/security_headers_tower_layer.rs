#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug)]
pub(super) struct SecurityHeadersTowerLayer {
    pub(super) content_security_policy: Option<super::HttpContentSecurityPolicy>,
    pub(super) forwarded_proto_trust: super::ForwardedProtoTrust,
}

impl<Service> tower::Layer<Service> for SecurityHeadersTowerLayer {
    type Service = super::SecurityHeadersService<Service>;

    fn layer(&self, inner: Service) -> Self::Service {
        super::SecurityHeadersService {
            content_security_policy: self.content_security_policy.clone(),
            forwarded_proto_trust: self.forwarded_proto_trust,
            inner,
        }
    }
}
