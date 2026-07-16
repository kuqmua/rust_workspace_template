#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OutboundHostPolicy {
    AllowPrivate,
    RejectPrivate,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OutboundUrlScheme {
    Http,
    Https,
    Rtsp,
    Rtsps,
}
#[derive(Clone, Copy, Debug)]
pub struct OutboundUrlTextRef<'value_lt>(&'value_lt str);
impl<'value_lt> From<&'value_lt str> for OutboundUrlTextRef<'value_lt> {
    fn from(value: &'value_lt str) -> Self {
        Self(value)
    }
}

#[derive(Clone)]
pub struct ReqwestOutboundUrl(reqwest::Url);
impl ReqwestOutboundUrl {
    #[must_use]
    pub fn scheme(&self) -> OutboundUrlScheme {
        match self.0.scheme() {
            str_constants::HTTPS => OutboundUrlScheme::Https,
            str_constants::RTSP => OutboundUrlScheme::Rtsp,
            str_constants::RTSPS => OutboundUrlScheme::Rtsps,
            _ => OutboundUrlScheme::Http,
        }
    }
}
impl std::fmt::Debug for ReqwestOutboundUrl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple(str_constants::OUTBOUND_URL)
            .field(&crate::redact_url_userinfo(self.0.as_str().into()))
            .finish()
    }
}

#[derive(Clone, Copy, Debug)]
pub struct StdOutboundIpAddr(std::net::IpAddr);
impl From<std::net::IpAddr> for StdOutboundIpAddr {
    fn from(value: std::net::IpAddr) -> Self {
        Self(value)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct OutboundUrlPolicy {
    host_policy: OutboundHostPolicy,
    schemes: &'static [OutboundUrlScheme],
}
impl OutboundUrlPolicy {
    #[must_use]
    pub const fn new(
        schemes: &'static [OutboundUrlScheme],
        host_policy: OutboundHostPolicy,
    ) -> Self {
        Self {
            host_policy,
            schemes,
        }
    }

    pub fn validate(
        self,
        value: OutboundUrlTextRef<'_>,
    ) -> Result<ReqwestOutboundUrl, OutboundUrlError> {
        if value.0.contains(['\0', '\r', '\n'])
            || contains_encoded_control(value) == OutboundAddressDisposition::Forbidden
        {
            return Err(OutboundUrlError::ControlCharacter);
        }
        let url = reqwest::Url::parse(value.0).map_err(|_error| OutboundUrlError::Invalid)?;
        if !self.schemes.iter().any(|scheme| match scheme {
            OutboundUrlScheme::Http => url.scheme() == str_constants::HTTP,
            OutboundUrlScheme::Https => url.scheme() == str_constants::HTTPS,
            OutboundUrlScheme::Rtsp => url.scheme() == str_constants::RTSP,
            OutboundUrlScheme::Rtsps => url.scheme() == str_constants::RTSPS,
        }) {
            return Err(OutboundUrlError::Scheme);
        }
        let host = url.host_str().ok_or(OutboundUrlError::MissingHost)?;
        if self.host_policy == OutboundHostPolicy::RejectPrivate
            && (host.eq_ignore_ascii_case(str_constants::LOCALHOST)
                || host
                    .to_ascii_lowercase()
                    .ends_with(str_constants::DOT_LOCALHOST))
        {
            return Err(OutboundUrlError::ForbiddenHost);
        }
        if self.host_policy == OutboundHostPolicy::RejectPrivate
            && host.parse::<std::net::IpAddr>().is_ok_and(|address| {
                outbound_address_disposition(StdOutboundIpAddr(address))
                    == OutboundAddressDisposition::Forbidden
            })
        {
            return Err(OutboundUrlError::ForbiddenHost);
        }
        Ok(ReqwestOutboundUrl(url))
    }

    pub fn validate_resolved_addresses(
        self,
        addresses: &[StdOutboundIpAddr],
    ) -> Result<(), OutboundUrlError> {
        if addresses.is_empty() {
            return Err(OutboundUrlError::MissingResolvedAddress);
        }
        if self.host_policy == OutboundHostPolicy::RejectPrivate
            && addresses.iter().any(|address| {
                outbound_address_disposition(*address) == OutboundAddressDisposition::Forbidden
            })
        {
            Err(OutboundUrlError::ForbiddenHost)
        } else {
            Ok(())
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
pub enum OutboundUrlError {
    #[error("outbound URL contains a forbidden control character")]
    ControlCharacter,
    #[error("outbound URL resolves to a forbidden address")]
    ForbiddenHost,
    #[error("outbound URL is invalid")]
    Invalid,
    #[error("outbound URL has no host")]
    MissingHost,
    #[error("outbound URL did not resolve to an address")]
    MissingResolvedAddress,
    #[error("outbound URL scheme is not allowed")]
    Scheme,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum OutboundAddressDisposition {
    Allowed,
    Forbidden,
}

#[allow(clippy::single_call_fn)] // keeps percent-encoded control recognition separate from URL parsing policy
fn contains_encoded_control(value: OutboundUrlTextRef<'_>) -> OutboundAddressDisposition {
    if value.0.as_bytes().windows(3usize).any(|window| {
        window.eq_ignore_ascii_case(str_constants::PERCENT_ENCODED_NUL)
            || window.eq_ignore_ascii_case(str_constants::PERCENT_ENCODED_CR)
            || window.eq_ignore_ascii_case(str_constants::PERCENT_ENCODED_LF)
    }) {
        OutboundAddressDisposition::Forbidden
    } else {
        OutboundAddressDisposition::Allowed
    }
}

fn outbound_address_disposition(address: StdOutboundIpAddr) -> OutboundAddressDisposition {
    let forbidden = match address.0 {
        std::net::IpAddr::V4(ipv4_address) => {
            ipv4_address.is_broadcast()
                || ipv4_address.is_link_local()
                || ipv4_address.is_loopback()
                || ipv4_address.is_multicast()
                || ipv4_address.is_private()
                || ipv4_address.is_unspecified()
        }
        std::net::IpAddr::V6(ipv6_address) => ipv6_address.to_ipv4_mapped().map_or_else(
            || {
                ipv6_address.is_loopback()
                    || ipv6_address.is_multicast()
                    || ipv6_address.is_unicast_link_local()
                    || ipv6_address.is_unique_local()
                    || ipv6_address.is_unspecified()
            },
            |mapped| {
                outbound_address_disposition(StdOutboundIpAddr(std::net::IpAddr::V4(mapped)))
                    == OutboundAddressDisposition::Forbidden
            },
        ),
    };
    if forbidden {
        OutboundAddressDisposition::Forbidden
    } else {
        OutboundAddressDisposition::Allowed
    }
}

#[cfg(test)]
mod tests {
    const POLICY: super::OutboundUrlPolicy = super::OutboundUrlPolicy::new(
        &[
            super::OutboundUrlScheme::Http,
            super::OutboundUrlScheme::Https,
        ],
        super::OutboundHostPolicy::RejectPrivate,
    );

    #[test]
    fn public_url_and_address_are_accepted() {
        let url = POLICY
            .validate(str_constants::TEST_PUBLIC_HTTPS_URL.into())
            .expect("a275c7bf");
        assert_eq!(url.scheme(), super::OutboundUrlScheme::Https);
        assert_eq!(
            POLICY.validate_resolved_addresses(&[std::net::IpAddr::V4(std::net::Ipv4Addr::new(
                8u8, 8u8, 8u8, 8u8
            ))
            .into(),]),
            Ok(())
        );
    }

    #[test]
    fn local_literal_hostname_and_encoded_control_are_rejected() {
        assert!(matches!(
            POLICY.validate(str_constants::HTTP_LOCALHOST.into()),
            Err(super::OutboundUrlError::ForbiddenHost)
        ));
        assert!(matches!(
            POLICY.validate(str_constants::TEST_LOOPBACK_HTTP_URL.into()),
            Err(super::OutboundUrlError::ForbiddenHost)
        ));
        assert!(matches!(
            POLICY.validate(str_constants::TEST_URL_WITH_ENCODED_NEWLINE.into()),
            Err(super::OutboundUrlError::ControlCharacter)
        ));
    }
}
