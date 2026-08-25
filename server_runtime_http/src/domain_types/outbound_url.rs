#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq)]
pub enum OutboundHostPolicy {
    AllowPrivate,
    RejectPrivate,
}

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq)]
pub enum OutboundUrlScheme {
    Http,
    Https,
    Rtsp,
    Rtsps,
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, newtype::FromInner)]
pub struct OutboundUrlTextRef<'value_lt>(&'value_lt str);

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, newtype::FromInner)]
pub struct ReqwestOutboundUrl(reqwest::Url);

impl ReqwestOutboundUrl {
    #[must_use]
    pub fn scheme(&self) -> OutboundUrlScheme {
        match self.0.scheme() {
            constants_str::HTTPS => OutboundUrlScheme::Https,
            constants_str::RTSP => OutboundUrlScheme::Rtsp,
            constants_str::RTSPS => OutboundUrlScheme::Rtsps,
            _ => OutboundUrlScheme::Http,
        }
    }
}

#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, Ord, PartialEq, PartialOrd,
)]
pub struct OutboundAllowedHost(String);
impl TryFrom<String> for OutboundAllowedHost {
    type Error = OutboundHostAllowlistError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty()
            || value.len() > 253usize
            || value.bytes().any(|byte| {
                !(byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'-' | b':' | b'[' | b']'))
            })
        {
            return Err(OutboundHostAllowlistError::InvalidHost);
        }
        Ok(Self(value.to_ascii_lowercase()))
    }
}

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq)]
pub struct OutboundHostAllowlist(
    bounded_types::domain_types::vector::BoundedVec<OutboundAllowedHost, 1, 64>,
);
impl TryFrom<Vec<OutboundAllowedHost>> for OutboundHostAllowlist {
    type Error = OutboundHostAllowlistError;
    fn try_from(mut value: Vec<OutboundAllowedHost>) -> Result<Self, Self::Error> {
        value.sort_unstable();
        value.dedup();
        bounded_types::domain_types::vector::BoundedVec::try_from(value)
            .map(Self)
            .map_err(|error| match error {
                bounded_types::domain_types::BoundedValueError::BelowMin { .. } => {
                    OutboundHostAllowlistError::Empty
                }
                bounded_types::domain_types::BoundedValueError::AboveMax { .. }
                | bounded_types::domain_types::BoundedValueError::InvalidBounds { .. } => {
                    OutboundHostAllowlistError::TooManyHosts
                }
            })
    }
}
impl OutboundHostAllowlist {
    pub fn validate(&self, url: &ReqwestOutboundUrl) -> Result<(), OutboundHostAllowlistError> {
        let host = url
            .0
            .host_str()
            .ok_or(OutboundHostAllowlistError::InvalidHost)?;
        if self
            .0
            .binary_search_by(|allowed| allowed.0.as_str().cmp(host))
            .is_ok()
        {
            Ok(())
        } else {
            Err(OutboundHostAllowlistError::HostNotAllowed)
        }
    }
}

#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error,
)]
pub enum OutboundHostAllowlistError {
    #[error("outbound host allowlist must not be empty")]
    Empty,
    #[error("outbound host is not present in the allowlist")]
    HostNotAllowed,
    #[error("outbound allowlist host is invalid")]
    InvalidHost,
    #[error("outbound host allowlist exceeds 64 entries")]
    TooManyHosts,
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

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, newtype::FromInner)]
pub struct OutboundIpAddr(std::net::IpAddr);

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug)]
#[allow(clippy::arbitrary_source_item_ordering)] // alignment order required by optimal_memory_layout takes precedence over alphabetical field order
pub struct OutboundUrlPolicy {
    schemes: &'static [OutboundUrlScheme],
    host_policy: OutboundHostPolicy,
}
impl OutboundUrlPolicy {
    #[must_use]
    pub const fn new(
        schemes: &'static [OutboundUrlScheme],
        host_policy: OutboundHostPolicy,
    ) -> Self {
        Self {
            schemes,
            host_policy,
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
        if !url.username().is_empty() || url.password().is_some() {
            return Err(OutboundUrlError::UserInfo);
        }
        if !self.schemes.iter().any(|scheme| match scheme {
            OutboundUrlScheme::Http => url.scheme() == constants_str::HTTP,
            OutboundUrlScheme::Https => url.scheme() == constants_str::HTTPS,
            OutboundUrlScheme::Rtsp => url.scheme() == constants_str::RTSP,
            OutboundUrlScheme::Rtsps => url.scheme() == constants_str::RTSPS,
        }) {
            return Err(OutboundUrlError::Scheme);
        }
        let host = url.host_str().ok_or(OutboundUrlError::MissingHost)?;
        if self.host_policy == OutboundHostPolicy::RejectPrivate
            && (host.eq_ignore_ascii_case(constants_str::LOCALHOST)
                || host
                    .to_ascii_lowercase()
                    .ends_with(constants_str::DOT_LOCALHOST))
        {
            return Err(OutboundUrlError::ForbiddenHost);
        }
        if self.host_policy == OutboundHostPolicy::RejectPrivate
            && host.parse::<std::net::IpAddr>().is_ok_and(|address| {
                outbound_address_disposition(OutboundIpAddr::from(address))
                    == OutboundAddressDisposition::Forbidden
            })
        {
            return Err(OutboundUrlError::ForbiddenHost);
        }
        Ok(ReqwestOutboundUrl::from(url))
    }

    pub fn validate_resolved_addresses(
        self,
        addresses: &[OutboundIpAddr],
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

#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error,
)]
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
    #[error("outbound URL must not contain user information")]
    UserInfo,
}

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq)]
enum OutboundAddressDisposition {
    Allowed,
    Forbidden,
}

#[allow(clippy::single_call_fn)] // keeps percent-encoded control recognition separate from URL parsing policy
fn contains_encoded_control(value: OutboundUrlTextRef<'_>) -> OutboundAddressDisposition {
    if value.0.as_bytes().windows(3usize).any(|window| {
        window.eq_ignore_ascii_case(constants_str::PERCENT_ENCODED_NUL)
            || window.eq_ignore_ascii_case(constants_str::PERCENT_ENCODED_CR)
            || window.eq_ignore_ascii_case(constants_str::PERCENT_ENCODED_LF)
    }) {
        OutboundAddressDisposition::Forbidden
    } else {
        OutboundAddressDisposition::Allowed
    }
}

fn outbound_address_disposition(address: OutboundIpAddr) -> OutboundAddressDisposition {
    let forbidden = match address.0 {
        std::net::IpAddr::V4(ipv4_address) => {
            let octets = ipv4_address.octets();
            ipv4_address.is_broadcast()
                || ipv4_address.is_link_local()
                || ipv4_address.is_loopback()
                || ipv4_address.is_multicast()
                || ipv4_address.is_private()
                || ipv4_address.is_unspecified()
                || octets[0] == constants_u8::ZERO
                || (octets[0] == 100u8 && (64u8..=127u8).contains(&octets[1]))
                || (octets[0] == 192u8
                    && octets[1] == constants_u8::ZERO
                    && octets[2] == constants_u8::ZERO)
                || (octets[0] == 192u8 && octets[1] == constants_u8::ZERO && octets[2] == 2u8)
                || (octets[0] == 198u8 && (octets[1] == 18u8 || octets[1] == 19u8))
                || (octets[0] == 198u8 && octets[1] == 51u8 && octets[2] == 100u8)
                || (octets[0] == 203u8 && octets[1] == constants_u8::ZERO && octets[2] == 113u8)
                || octets[0] >= 240u8
        }
        std::net::IpAddr::V6(ipv6_address) => ipv6_address.to_ipv4_mapped().map_or_else(
            || {
                ipv6_address.is_loopback()
                    || ipv6_address.is_multicast()
                    || ipv6_address.is_unicast_link_local()
                    || ipv6_address.is_unique_local()
                    || ipv6_address.is_unspecified()
                    || ipv6_address.segments()[..2usize] == [0x2001u16, 0x0db8u16]
            },
            |mapped| {
                outbound_address_disposition(OutboundIpAddr::from(std::net::IpAddr::V4(mapped)))
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
            .validate(constants_str::TEST_PUBLIC_HTTPS_URL.into())
            .expect("a275c7bf public_url_and_address_are_accepted invariant must hold");
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
            POLICY.validate(constants_str::HTTP_LOCALHOST.into()),
            Err(super::OutboundUrlError::ForbiddenHost)
        ));
        assert!(matches!(
            POLICY.validate(constants_str::TEST_LOOPBACK_HTTP_URL.into()),
            Err(super::OutboundUrlError::ForbiddenHost)
        ));
        assert!(matches!(
            POLICY.validate(constants_str::TEST_URL_WITH_ENCODED_NEWLINE.into()),
            Err(super::OutboundUrlError::ControlCharacter)
        ));
    }

    #[test]
    fn non_global_special_addresses_are_rejected() {
        assert!(
            [
                std::net::IpAddr::V4(std::net::Ipv4Addr::new(
                    constants_u8::ZERO,
                    constants_u8::ZERO,
                    constants_u8::ZERO,
                    1u8
                )),
                std::net::IpAddr::V4(std::net::Ipv4Addr::new(
                    100u8,
                    64u8,
                    constants_u8::ZERO,
                    1u8
                )),
                std::net::IpAddr::V4(std::net::Ipv4Addr::new(192u8, constants_u8::ZERO, 2u8, 1u8)),
                std::net::IpAddr::V4(std::net::Ipv4Addr::new(
                    198u8,
                    18u8,
                    constants_u8::ZERO,
                    1u8
                )),
                std::net::IpAddr::V4(std::net::Ipv4Addr::new(198u8, 51u8, 100u8, 1u8)),
                std::net::IpAddr::V4(std::net::Ipv4Addr::new(
                    203u8,
                    constants_u8::ZERO,
                    113u8,
                    1u8
                )),
                std::net::IpAddr::V4(std::net::Ipv4Addr::new(
                    240u8,
                    constants_u8::ZERO,
                    constants_u8::ZERO,
                    1u8
                )),
                std::net::IpAddr::V6(std::net::Ipv6Addr::new(
                    0x2001u16,
                    0x0db8u16,
                    constants_u16::ZERO,
                    constants_u16::ZERO,
                    constants_u16::ZERO,
                    constants_u16::ZERO,
                    constants_u16::ZERO,
                    1u16,
                )),
            ]
            .into_iter()
            .all(|address| {
                matches!(
                    POLICY.validate_resolved_addresses(&[address.into()]),
                    Err(super::OutboundUrlError::ForbiddenHost)
                )
            })
        );
    }

    #[test]
    fn allowlist_requires_exact_host_and_url_rejects_userinfo() {
        let allowed_host = super::OutboundAllowedHost::try_from(String::from(
            constants_str::TEST_PUBLIC_HOST,
        ))
        .expect(
            "3e5decb1 allowlist_requires_exact_host_and_url_rejects_userinfo invariant must hold",
        );
        let allowlist = super::OutboundHostAllowlist::try_from(vec![allowed_host]).expect(
            "920be78f allowlist_requires_exact_host_and_url_rejects_userinfo invariant must hold",
        );
        let allowed = POLICY
            .validate(constants_str::TEST_PUBLIC_HTTPS_URL.into())
            .expect("27a67a96 allowlist_requires_exact_host_and_url_rejects_userinfo invariant must hold");
        assert_eq!(allowlist.validate(&allowed), Ok(()));
        let other = POLICY
            .validate(constants_str::TEST_OTHER_PUBLIC_HTTPS_URL.into())
            .expect("b3981504 allowlist_requires_exact_host_and_url_rejects_userinfo invariant must hold");
        assert_eq!(
            allowlist.validate(&other),
            Err(super::OutboundHostAllowlistError::HostNotAllowed)
        );
        assert!(matches!(
            POLICY.validate(constants_str::TEST_PUBLIC_HTTPS_URL_WITH_USERINFO.into()),
            Err(super::OutboundUrlError::UserInfo)
        ));
    }
}
