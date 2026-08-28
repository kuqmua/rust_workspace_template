#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug)]
pub struct SecurityHeadersLayer {
    content_security_policy: Option<super::HttpContentSecurityPolicy>,
    forwarded_proto_trust: super::ForwardedProtoTrust,
}

impl From<super::ForwardedProtoTrust> for SecurityHeadersLayer {
    fn from(value: super::ForwardedProtoTrust) -> Self {
        Self {
            content_security_policy: None,
            forwarded_proto_trust: value,
        }
    }
}

impl SecurityHeadersLayer {
    #[must_use]
    pub fn apply(self, router: crate::AxumRouter) -> crate::AxumRouter {
        crate::AxumRouter::from(axum::Router::from(router).layer(
            super::SecurityHeadersTowerLayer {
                content_security_policy: self.content_security_policy,
                forwarded_proto_trust: self.forwarded_proto_trust,
            },
        ))
    }

    #[must_use]
    pub fn with_content_security_policy(mut self, value: super::HttpContentSecurityPolicy) -> Self {
        self.content_security_policy = Some(value);
        self
    }
}
