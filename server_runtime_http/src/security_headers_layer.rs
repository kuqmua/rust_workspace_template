#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug)]
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
    pub fn apply(self, router: crate::axum_router::AxumRouter) -> crate::axum_router::AxumRouter {
        crate::axum_router::AxumRouter::from(axum::Router::from(router).layer(
            crate::security_headers_tower_layer::SecurityHeadersTowerLayer {
                content_security_policy: self.content_security_policy,
                forwarded_proto_trust: self.forwarded_proto_trust,
            },
        ))
    }

    #[must_use]
    pub fn with_content_security_policy(
        mut self,
        value: crate::http_content_security_policy::HttpContentSecurityPolicy,
    ) -> Self {
        self.content_security_policy = Some(value);
        self
    }
}
