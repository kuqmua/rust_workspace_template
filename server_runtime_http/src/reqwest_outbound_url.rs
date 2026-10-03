#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    proc_macro_newtype_from_inner::FromInner,
)]
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
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_tuple(constants_str::OUTBOUND_URL)
            .field(&crate::redact_url_userinfo::redact_url_userinfo(
                self.0.as_str().into(),
            ))
            .finish()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_outbound_url_preserves_scheme_and_host_while_redacting_credentials() {
        assert!(
            [
                (
                    constants_str::HTTP,
                    crate::outbound_url_scheme::OutboundUrlScheme::Http
                ),
                (
                    constants_str::HTTPS,
                    crate::outbound_url_scheme::OutboundUrlScheme::Https
                ),
                (
                    constants_str::RTSP,
                    crate::outbound_url_scheme::OutboundUrlScheme::Rtsp
                ),
                (
                    constants_str::RTSPS,
                    crate::outbound_url_scheme::OutboundUrlScheme::Rtsps
                ),
            ]
            .into_iter()
            .all(|(scheme, expected)| {
                let text = constants_str::TEST_URL_WITH_CREDENTIALS.replacen(
                    constants_str::HTTPS,
                    scheme,
                    1usize,
                );
                reqwest::Url::parse(&text).is_ok_and(|url| {
                    let wrapped =
                        crate::reqwest_outbound_url::ReqwestOutboundUrl::from(url.clone());
                    let debug = format!("{wrapped:?}");
                    wrapped.scheme() == expected
                        && wrapped.host_str() == url.host_str()
                        && !debug.contains(url.username())
                        && url
                            .password()
                            .is_none_or(|password| !debug.contains(password))
                        && debug.contains(constants_str::LOCALHOST)
                })
            })
        );
    }
}
