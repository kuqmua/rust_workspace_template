#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Clone, Debug)]
pub struct SecurityHeadersLayer {
    content_security_policy: Option<crate::http_content_security_policy::HttpContentSecurityPolicy>,
    forwarded_proto_trust: crate::forwarded_proto_trust::ForwardedProtoTrust,
}

impl From<crate::forwarded_proto_trust::ForwardedProtoTrust> for SecurityHeadersLayer {
    fn from(value: crate::forwarded_proto_trust::ForwardedProtoTrust) -> Self {
        Self {
            content_security_policy: None,
            forwarded_proto_trust: value,
        }
    }
}

impl SecurityHeadersLayer {
    #[must_use]
    pub fn apply(
        self,
        axum_router: crate::axum_router::AxumRouter,
    ) -> crate::axum_router::AxumRouter {
        crate::axum_router::AxumRouter::from(axum::Router::from(axum_router).layer(
            crate::security_headers_tower_layer::SecurityHeadersTowerLayer::new(
                self.content_security_policy,
                self.forwarded_proto_trust,
            ),
        ))
    }

    #[must_use]
    pub fn with_content_security_policy(
        mut self,
        http_content_security_policy: crate::http_content_security_policy::HttpContentSecurityPolicy,
    ) -> Self {
        self.content_security_policy = Some(http_content_security_policy);
        self
    }
}
