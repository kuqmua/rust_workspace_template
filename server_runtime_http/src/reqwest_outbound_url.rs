#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, newtype::FromInner)]
pub struct ReqwestOutboundUrl(reqwest::Url);

impl ReqwestOutboundUrl {
    pub(crate) fn host_str(&self) -> Option<&str> {
        self.0.host_str()
    }

    #[must_use]
    pub fn scheme(&self) -> crate::outbound_url_scheme::OutboundUrlScheme {
        match self.0.scheme() {
            constants_str::HTTPS => crate::outbound_url_scheme::OutboundUrlScheme::Https,
            constants_str::RTSP => crate::outbound_url_scheme::OutboundUrlScheme::Rtsp,
            constants_str::RTSPS => crate::outbound_url_scheme::OutboundUrlScheme::Rtsps,
            _ => crate::outbound_url_scheme::OutboundUrlScheme::Http,
        }
    }
}

impl std::fmt::Debug for ReqwestOutboundUrl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple(constants_str::OUTBOUND_URL)
            .field(&crate::redact_url_userinfo::redact_url_userinfo(
                self.0.as_str().into(),
            ))
            .finish()
    }
}
