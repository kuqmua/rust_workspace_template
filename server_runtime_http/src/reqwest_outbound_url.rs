#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, newtype::FromInner)]
pub struct ReqwestOutboundUrl(pub(super) reqwest::Url);

impl ReqwestOutboundUrl {
    #[must_use]
    pub fn scheme(&self) -> super::OutboundUrlScheme {
        match self.0.scheme() {
            constants_str::HTTPS => super::OutboundUrlScheme::Https,
            constants_str::RTSP => super::OutboundUrlScheme::Rtsp,
            constants_str::RTSPS => super::OutboundUrlScheme::Rtsps,
            _ => super::OutboundUrlScheme::Http,
        }
    }
}

impl std::fmt::Debug for ReqwestOutboundUrl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple(constants_str::OUTBOUND_URL)
            .field(&crate::domain_types::redact_url_userinfo(
                self.0.as_str().into(),
            ))
            .finish()
    }
}
