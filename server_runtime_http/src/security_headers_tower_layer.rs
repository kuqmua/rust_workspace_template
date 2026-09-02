#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, proc_macro_new::New,
)]
#[constructor(pub(crate))]
pub(super) struct SecurityHeadersTowerLayer {
    content_security_policy: Option<crate::http_content_security_policy::HttpContentSecurityPolicy>,
    forwarded_proto_trust: crate::forwarded_proto_trust::ForwardedProtoTrust,
}

impl<Service> tower::Layer<Service> for SecurityHeadersTowerLayer {
    type Service = crate::security_headers_service::SecurityHeadersService<Service>;

    fn layer(&self, inner: Service) -> Self::Service {
        crate::security_headers_service::SecurityHeadersService::new(
            self.content_security_policy.clone(),
            self.forwarded_proto_trust,
            inner,
        )
    }
}
