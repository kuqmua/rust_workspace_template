const MAX_FORWARDED_HEADER_BYTES: usize = 4096;
const MAX_FORWARDED_ENTRIES: usize = 32;
#[derive(Clone, Copy, Debug)]
pub struct HttpHeaderMapRef<'lt>(&'lt http::HeaderMap);
impl<'lt> From<&'lt http::HeaderMap> for HttpHeaderMapRef<'lt> {
    fn from(value: &'lt http::HeaderMap) -> Self {
        Self(value)
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StdSocketAddr(std::net::SocketAddr);
impl From<std::net::SocketAddr> for StdSocketAddr {
    fn from(value: std::net::SocketAddr) -> Self {
        Self(value)
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StdResolvedClientIp(std::net::IpAddr);
impl AsRef<std::net::IpAddr> for StdResolvedClientIp {
    fn as_ref(&self) -> &std::net::IpAddr {
        &self.0
    }
}
impl std::fmt::Display for StdResolvedClientIp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, f)
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TrustedProxyRange {
    network: StdIpAddr,
    prefix_bits: StdTrustedProxyPrefixBits,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct StdIpAddr(std::net::IpAddr);
impl From<std::net::IpAddr> for StdIpAddr {
    fn from(value: std::net::IpAddr) -> Self {
        Self(value)
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct StdRangeContains(bool);
impl StdRangeContains {
    const fn get(self) -> bool {
        self.0
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct StdTrustedProxyPrefixBits(u8);
impl From<u8> for StdTrustedProxyPrefixBits {
    fn from(value: u8) -> Self {
        Self(value)
    }
}
#[derive(Debug)]
pub struct StdAddrParseError(std::net::AddrParseError);
impl std::fmt::Display for StdAddrParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, f)
    }
}
impl std::error::Error for StdAddrParseError {}
impl From<std::net::AddrParseError> for StdAddrParseError {
    fn from(value: std::net::AddrParseError) -> Self {
        Self(value)
    }
}
#[derive(Debug)]
pub struct StdParseIntError(std::num::ParseIntError);
impl std::fmt::Display for StdParseIntError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, f)
    }
}
impl std::error::Error for StdParseIntError {}
impl From<std::num::ParseIntError> for StdParseIntError {
    fn from(value: std::num::ParseIntError) -> Self {
        Self(value)
    }
}
#[derive(Debug, thiserror::Error)]
pub enum TrustedProxyRangeParseError {
    #[error("trusted proxy address is invalid")]
    InvalidAddress {
        #[source]
        source: StdAddrParseError,
    },
    #[error("trusted proxy prefix is invalid")]
    InvalidPrefix {
        #[source]
        source: StdParseIntError,
    },
    #[error("trusted proxy range must use address/prefix notation")]
    MissingPrefix,
    #[error("trusted proxy prefix exceeds address width")]
    PrefixExceedsAddressWidth,
}
impl TryFrom<String> for TrustedProxyRange {
    type Error = TrustedProxyRangeParseError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        let Some((address_text, prefix_text)) = value.split_once('/') else {
            return Err(TrustedProxyRangeParseError::MissingPrefix);
        };
        let network = address_text.parse::<std::net::IpAddr>().map_err(|source| {
            TrustedProxyRangeParseError::InvalidAddress {
                source: StdAddrParseError::from(source),
            }
        })?;
        let prefix_bits = prefix_text.parse::<u8>().map_err(|source| {
            TrustedProxyRangeParseError::InvalidPrefix {
                source: StdParseIntError::from(source),
            }
        })?;
        let address_width = match network {
            std::net::IpAddr::V4(_) => 32u8,
            std::net::IpAddr::V6(_) => 128u8,
        };
        if prefix_bits > address_width {
            return Err(TrustedProxyRangeParseError::PrefixExceedsAddressWidth);
        }
        Ok(Self {
            network: StdIpAddr::from(network),
            prefix_bits: StdTrustedProxyPrefixBits::from(prefix_bits),
        })
    }
}
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct TrustedProxyRanges(Vec<TrustedProxyRange>);
impl From<Vec<TrustedProxyRange>> for TrustedProxyRanges {
    fn from(value: Vec<TrustedProxyRange>) -> Self {
        Self(value)
    }
}
impl TrustedProxyRange {
    fn contains(self, candidate_ip: StdIpAddr) -> StdRangeContains {
        StdRangeContains(match (self.network.0, candidate_ip.0) {
            (std::net::IpAddr::V4(network), std::net::IpAddr::V4(candidate_v4)) => {
                let shift = 32u32.saturating_sub(u32::from(self.prefix_bits.0));
                let mask = u32::MAX.checked_shl(shift).unwrap_or(0u32);
                u32::from(network) & mask == u32::from(candidate_v4) & mask
            }
            (std::net::IpAddr::V6(network), std::net::IpAddr::V6(candidate_v6)) => {
                let shift = 128u32.saturating_sub(u32::from(self.prefix_bits.0));
                let mask = u128::MAX.checked_shl(shift).unwrap_or(0u128);
                u128::from(network) & mask == u128::from(candidate_v6) & mask
            }
            _ => false,
        })
    }
}
impl TrustedProxyRanges {
    fn contains(&self, candidate: StdIpAddr) -> StdRangeContains {
        StdRangeContains(self.0.iter().any(|range| range.contains(candidate).get()))
    }
}
#[must_use]
pub fn resolve_client_ip(
    headers: HttpHeaderMapRef<'_>,
    peer: StdSocketAddr,
    trusted_proxy_ranges: &TrustedProxyRanges,
) -> StdResolvedClientIp {
    let peer_ip = peer.0.ip();
    if !trusted_proxy_ranges
        .contains(StdIpAddr::from(peer_ip))
        .get()
    {
        return StdResolvedClientIp(peer_ip);
    }
    let parsed_forwarded_ip = || {
        let values = headers
            .0
            .get_all(str_constants::runtime::FORWARDED_FOR_HEADER_NAME);
        let mut iter = values.iter();
        let value = iter.next()?;
        if iter.next().is_some() || value.as_bytes().len() > MAX_FORWARDED_HEADER_BYTES {
            return None;
        }
        let value_text = value.to_str().ok()?;
        let entries = value_text.split(',').map(str::trim).collect::<Vec<&str>>();
        if entries.is_empty() || entries.len() > MAX_FORWARDED_ENTRIES {
            return None;
        }
        let parsed = entries
            .iter()
            .map(|entry| entry.parse::<std::net::IpAddr>())
            .collect::<Result<Vec<std::net::IpAddr>, std::net::AddrParseError>>()
            .ok()?;
        parsed
            .iter()
            .rev()
            .find(|candidate| {
                !trusted_proxy_ranges
                    .contains(StdIpAddr::from(**candidate))
                    .get()
            })
            .copied()
            .or_else(|| parsed.first().copied())
    };
    let parsed_real_ip = || {
        let values = headers
            .0
            .get_all(str_constants::runtime::REAL_IP_HEADER_NAME);
        let mut iter = values.iter();
        let value = iter.next()?;
        if iter.next().is_some() || value.as_bytes().len() > MAX_FORWARDED_HEADER_BYTES {
            return None;
        }
        value.to_str().ok()?.trim().parse::<std::net::IpAddr>().ok()
    };
    StdResolvedClientIp(
        parsed_forwarded_ip()
            .or_else(parsed_real_ip)
            .unwrap_or(peer_ip),
    )
}
#[cfg(test)]
mod tests {
    fn range(value: &str) -> super::TrustedProxyRange {
        super::TrustedProxyRange::try_from(value.to_owned()).expect("46cc9e0a")
    }
    fn resolved(
        headers: &http::HeaderMap,
        peer_value: &str,
        ranges: Vec<super::TrustedProxyRange>,
    ) -> String {
        super::resolve_client_ip(
            super::HttpHeaderMapRef::from(headers),
            super::StdSocketAddr::from(
                peer_value
                    .parse::<std::net::SocketAddr>()
                    .expect("262819a8"),
            ),
            &super::TrustedProxyRanges::from(ranges),
        )
        .to_string()
    }
    #[test]
    fn untrusted_peer_cannot_spoof_forwarded_header() {
        let mut headers = http::HeaderMap::new();
        let _previous = headers.insert(
            str_constants::runtime::FORWARDED_FOR_HEADER_NAME,
            http::HeaderValue::from_static(str_constants::expr::S_0222),
        );
        assert_eq!(
            resolved(&headers, "198.51.100.2:80", Vec::new()),
            "198.51.100.2"
        );
    }
    #[test]
    fn trusted_chain_resolves_first_untrusted_address_from_right() {
        let mut headers = http::HeaderMap::new();
        let _previous = headers.insert(
            str_constants::runtime::FORWARDED_FOR_HEADER_NAME,
            http::HeaderValue::from_static(str_constants::expr::S_0226),
        );
        assert_eq!(
            resolved(&headers, "10.0.0.10:80", vec![range("10.0.0.0/24")]),
            "203.0.113.7"
        );
    }
    #[test]
    fn ipv4_range_does_not_trust_ipv6_peer() {
        let mut headers = http::HeaderMap::new();
        let _previous = headers.insert(
            str_constants::runtime::FORWARDED_FOR_HEADER_NAME,
            http::HeaderValue::from_static(str_constants::expr::S_0225),
        );
        assert_eq!(
            resolved(&headers, "[::1]:80", vec![range("127.0.0.0/8")]),
            "::1"
        );
    }
    #[test]
    fn malformed_and_multiple_headers_fall_back_to_peer() {
        let mut malformed_headers = http::HeaderMap::new();
        let _inserted_malformed = malformed_headers.append(
            str_constants::runtime::FORWARDED_FOR_HEADER_NAME,
            http::HeaderValue::from_static(str_constants::expr::S_1560),
        );
        assert_eq!(
            resolved(
                &malformed_headers,
                "10.0.0.10:80",
                vec![range("10.0.0.0/24")]
            ),
            "10.0.0.10"
        );
        let mut mixed_headers = http::HeaderMap::new();
        let _inserted_mixed = mixed_headers.append(
            str_constants::runtime::FORWARDED_FOR_HEADER_NAME,
            http::HeaderValue::from_static(str_constants::expr::S_0223),
        );
        assert_eq!(
            resolved(&mixed_headers, "10.0.0.10:80", vec![range("10.0.0.0/24")]),
            "10.0.0.10"
        );
        let mut headers = http::HeaderMap::new();
        let _inserted_first = headers.append(
            str_constants::runtime::FORWARDED_FOR_HEADER_NAME,
            http::HeaderValue::from_static(str_constants::expr::S_0222),
        );
        let _inserted_second = headers.append(
            str_constants::runtime::FORWARDED_FOR_HEADER_NAME,
            http::HeaderValue::from_static(str_constants::expr::S_0224),
        );
        assert_eq!(
            resolved(&headers, "10.0.0.10:80", vec![range("10.0.0.0/24")]),
            "10.0.0.10"
        );
    }
    #[test]
    fn oversized_header_falls_back_without_reflecting_input() {
        let oversized = str_constants::expr::S_0167
            .repeat(super::MAX_FORWARDED_HEADER_BYTES.saturating_add(1usize));
        let mut headers = http::HeaderMap::new();
        let _previous = headers.insert(
            str_constants::runtime::FORWARDED_FOR_HEADER_NAME,
            http::HeaderValue::from_str(oversized.as_str()).expect("6353255d"),
        );
        assert_eq!(
            resolved(&headers, "10.0.0.10:80", vec![range("10.0.0.0/24")]),
            "10.0.0.10"
        );
    }
    #[test]
    fn prefixes_are_validated() {
        assert!(matches!(
            super::TrustedProxyRange::try_from("127.0.0.1/33".to_owned()),
            Err(super::TrustedProxyRangeParseError::PrefixExceedsAddressWidth)
        ));
        assert!(matches!(
            super::TrustedProxyRange::try_from("::1/129".to_owned()),
            Err(super::TrustedProxyRangeParseError::PrefixExceedsAddressWidth)
        ));
    }
}
