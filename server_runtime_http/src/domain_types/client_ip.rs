const MAX_FORWARDED_HEADER_BYTES: usize = 4096;
const MAX_FORWARDED_ENTRIES: usize = 32;
const TRUSTED_PROXY_RANGES_MAX_ITEMS: usize = 128usize;
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, newtype::FromInner)]
pub struct HttpHeaderMapRef<'lt>(&'lt http::HeaderMap);

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, newtype::FromInner)]
pub struct TrustedProxyRangesTextRef<'text_lt>(&'text_lt str);

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    newtype::Display,
    newtype::FromInner,
)]
pub struct ClientSocketAddr(std::net::SocketAddr);

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    newtype::AsRefOwned,
    newtype::Display,
    newtype::FromInner,
)]
pub struct ResolvedClientIpAddr(std::net::IpAddr);
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq)]
pub struct TrustedProxyRange {
    network: IpnetNetwork,
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    newtype::FromInner,
)]
struct IpnetNetwork(ipnet::IpNet);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    newtype::FromInner,
)]
struct ParsedIpAddr(std::net::IpAddr);

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    newtype::FromInner,
)]
struct StdRangeContains(bool);

impl StdRangeContains {
    const fn get(self) -> bool {
        self.0
    }
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error, newtype::FromInner,
)]
#[error("{0}")]
pub struct ClientAddrParseError(std::net::AddrParseError);

#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error, newtype::FromInner,
)]
#[error("{0}")]
pub struct ParseIntError(std::num::ParseIntError);

#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
pub enum TrustedProxyRangeParseError {
    #[error("trusted proxy address is invalid")]
    InvalidAddress {
        #[source]
        source: ClientAddrParseError,
    },
    #[error("trusted proxy prefix is invalid")]
    InvalidPrefix {
        #[source]
        source: ParseIntError,
    },
    #[error("trusted proxy range must use address/prefix notation")]
    MissingPrefix,
    #[error("trusted proxy prefix exceeds address width")]
    PrefixExceedsAddressWidth,
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
pub enum TrustedProxyRangesParseError {
    #[error("trusted proxy range is invalid: {0}")]
    Range(TrustedProxyRangeParseError),
    #[error("trusted proxy range list is invalid: {0}")]
    Ranges(TrustedProxyRangesError),
}
impl TryFrom<String> for TrustedProxyRange {
    type Error = TrustedProxyRangeParseError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        let Some((address_text, prefix_text)) = value.split_once('/') else {
            return Err(TrustedProxyRangeParseError::MissingPrefix);
        };
        let network_address = address_text.parse::<std::net::IpAddr>().map_err(|source| {
            TrustedProxyRangeParseError::InvalidAddress {
                source: ClientAddrParseError::from(source),
            }
        })?;
        let prefix_bits = prefix_text.parse::<u8>().map_err(|source| {
            TrustedProxyRangeParseError::InvalidPrefix {
                source: ParseIntError::from(source),
            }
        })?;
        let Ok(network) = ipnet::IpNet::new(network_address, prefix_bits) else {
            return Err(TrustedProxyRangeParseError::PrefixExceedsAddressWidth);
        };
        Ok(Self {
            network: IpnetNetwork::from(network),
        })
    }
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Default, Eq, PartialEq)]
pub struct TrustedProxyRanges(
    bounded_types::domain_types::vector::BoundedVec<
        TrustedProxyRange,
        0,
        TRUSTED_PROXY_RANGES_MAX_ITEMS,
    >,
);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error,
)]
#[error("trusted proxy range list exceeds its maximum item count")]
pub struct TrustedProxyRangesError;
impl From<bounded_types::domain_types::BoundedValueError> for TrustedProxyRangesError {
    fn from(_value: bounded_types::domain_types::BoundedValueError) -> Self {
        Self
    }
}
impl TryFrom<Vec<TrustedProxyRange>> for TrustedProxyRanges {
    type Error = TrustedProxyRangesError;
    fn try_from(value: Vec<TrustedProxyRange>) -> Result<Self, Self::Error> {
        bounded_types::domain_types::vector::BoundedVec::try_from(value)
            .map(Self)
            .map_err(TrustedProxyRangesError::from)
    }
}
impl TrustedProxyRange {
    fn contains(self, candidate_ip: ParsedIpAddr) -> StdRangeContains {
        StdRangeContains::from(self.network.0.contains(&candidate_ip.0))
    }
}
impl TrustedProxyRanges {
    fn contains(&self, candidate: ParsedIpAddr) -> StdRangeContains {
        StdRangeContains::from(self.0.iter().any(|range| range.contains(candidate).get()))
    }
}
#[must_use]
pub fn resolve_client_ip(
    headers: HttpHeaderMapRef<'_>,
    peer: ClientSocketAddr,
    trusted_proxy_ranges: &TrustedProxyRanges,
) -> ResolvedClientIpAddr {
    let peer_ip = peer.0.ip();
    if !trusted_proxy_ranges
        .contains(ParsedIpAddr::from(peer_ip))
        .get()
    {
        return ResolvedClientIpAddr::from(peer_ip);
    }
    let parsed_forwarded_ip = || {
        let values = headers
            .0
            .get_all(constants_str::RUNTIME_FORWARDED_FOR_HEADER_NAME);
        let mut iter = values.iter();
        let value = iter.next()?;
        if iter.next().is_some() || value.as_bytes().len() > MAX_FORWARDED_HEADER_BYTES {
            return None;
        }
        let value_text = value.to_str().ok()?;
        let (count, first, rightmost_untrusted) = value_text.split(',').map(str::trim).try_fold(
            (constants_usize::ZERO, None, None),
            |(count, first, rightmost_untrusted), entry| {
                if count >= MAX_FORWARDED_ENTRIES {
                    return None;
                }
                let parsed = entry.parse::<std::net::IpAddr>().ok()?;
                let next_first = first.or(Some(parsed));
                let next_rightmost_untrusted = if trusted_proxy_ranges
                    .contains(ParsedIpAddr::from(parsed))
                    .get()
                {
                    rightmost_untrusted
                } else {
                    Some(parsed)
                };
                Some((
                    count.saturating_add(constants_usize::ONE),
                    next_first,
                    next_rightmost_untrusted,
                ))
            },
        )?;
        (count > constants_usize::ZERO)
            .then_some(rightmost_untrusted.or(first))
            .flatten()
    };
    let parsed_real_ip = || {
        let values = headers
            .0
            .get_all(constants_str::RUNTIME_REAL_IP_HEADER_NAME);
        let mut iter = values.iter();
        let value = iter.next()?;
        if iter.next().is_some() || value.as_bytes().len() > MAX_FORWARDED_HEADER_BYTES {
            return None;
        }
        value.to_str().ok()?.trim().parse::<std::net::IpAddr>().ok()
    };
    ResolvedClientIpAddr::from(
        parsed_forwarded_ip()
            .or_else(parsed_real_ip)
            .unwrap_or(peer_ip),
    )
}

pub fn parse_trusted_proxy_ranges(
    value: TrustedProxyRangesTextRef<'_>,
) -> Result<TrustedProxyRanges, TrustedProxyRangesParseError> {
    if value.0.trim().is_empty() {
        return Ok(TrustedProxyRanges::default());
    }
    let ranges = value
        .0
        .split(',')
        .map(str::trim)
        .map(|item| {
            TrustedProxyRange::try_from(item.to_owned())
                .map_err(TrustedProxyRangesParseError::Range)
        })
        .collect::<Result<Vec<TrustedProxyRange>, TrustedProxyRangesParseError>>()?;
    TrustedProxyRanges::try_from(ranges).map_err(TrustedProxyRangesParseError::Ranges)
}
#[must_use]
pub fn resolve_header_text<'header>(
    headers: HttpHeaderMapRef<'header>,
    name: &crate::domain_types::HttpHeaderName,
    maximum: crate::domain_types::HttpHeaderTextMaximumBytes,
) -> crate::domain_types::HttpHeaderTextResolution<'header> {
    let Some(value) = headers.0.get(name.as_ref()) else {
        return crate::domain_types::HttpHeaderTextResolution::Missing;
    };
    let bytes = value.as_bytes();
    if bytes.len() > usize::from(maximum) {
        return crate::domain_types::HttpHeaderTextResolution::ExceedsMaximumBytes {
            actual_bytes: crate::domain_types::HttpHeaderTextBytes::from(bytes.len()),
        };
    }
    match value.to_str() {
        Ok(text) => crate::domain_types::HttpHeaderTextResolution::Value(
            crate::domain_types::HttpHeaderTextRef::from(text.trim()),
        ),
        Err(_error) => crate::domain_types::HttpHeaderTextResolution::InvalidText,
    }
}

#[cfg(test)]
mod tests {
    fn range(value: &str) -> super::TrustedProxyRange {
        super::TrustedProxyRange::try_from(value.to_owned())
            .expect("46cc9e0a range invariant must hold")
    }
    fn resolved(
        headers: &http::HeaderMap,
        peer_value: &str,
        ranges: Vec<super::TrustedProxyRange>,
    ) -> String {
        super::resolve_client_ip(
            super::HttpHeaderMapRef::from(headers),
            super::ClientSocketAddr::from(
                peer_value
                    .parse::<std::net::SocketAddr>()
                    .expect("262819a8 resolved invariant must hold"),
            ),
            &super::TrustedProxyRanges::try_from(ranges)
                .expect("38546d0b resolved invariant must hold"),
        )
        .to_string()
    }
    #[test]
    fn trusted_proxy_ranges_reject_oversized_lists() {
        let item = range(constants_str::VALUE_127_0_0_1_32);
        let values =
            vec![item; super::TRUSTED_PROXY_RANGES_MAX_ITEMS.saturating_add(constants_usize::ONE)];
        assert_eq!(
            super::TrustedProxyRanges::try_from(values),
            Err(super::TrustedProxyRangesError)
        );
    }
    #[test]
    fn trusted_proxy_ranges_text_parses_comma_separated_ranges() {
        let ranges = super::parse_trusted_proxy_ranges(super::TrustedProxyRangesTextRef::from(
            constants_str::VALUE_127_0_0_1_32_PATH_1_128,
        ))
        .expect(
            "60ad1a64 trusted_proxy_ranges_text_parses_comma_separated_ranges invariant must hold",
        );
        assert_eq!(
            super::resolve_client_ip(
                super::HttpHeaderMapRef::from(&http::HeaderMap::new()),
                super::ClientSocketAddr::from(
                    "127.0.0.1:8080"
                        .parse::<std::net::SocketAddr>()
                        .expect("a6f1a8f9 trusted_proxy_ranges_text_parses_comma_separated_ranges invariant must hold")
                ),
                &ranges,
            )
            .to_string(),
            "127.0.0.1"
        );
    }
    #[test]
    fn trusted_proxy_ranges_text_rejects_empty_list_entries() {
        assert!(matches!(
            super::parse_trusted_proxy_ranges(super::TrustedProxyRangesTextRef::from(
                "127.0.0.1/32,,::1/128",
            )),
            Err(super::TrustedProxyRangesParseError::Range(
                super::TrustedProxyRangeParseError::MissingPrefix
            ))
        ));
        let empty = super::parse_trusted_proxy_ranges(super::TrustedProxyRangesTextRef::from(" "))
            .expect(
                "639128ba trusted_proxy_ranges_text_rejects_empty_list_entries invariant must hold",
            );
        assert_eq!(empty, super::TrustedProxyRanges::default());
    }
    #[test]
    fn untrusted_peer_cannot_spoof_forwarded_header() {
        let mut headers = http::HeaderMap::new();
        let _previous = headers.insert(
            constants_str::RUNTIME_FORWARDED_FOR_HEADER_NAME,
            http::HeaderValue::from_static(constants_str::VALUE_203_0_113_1),
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
            constants_str::RUNTIME_FORWARDED_FOR_HEADER_NAME,
            http::HeaderValue::from_static(constants_str::VALUE_203_0_113_7_10_0_0_8_10_0_0),
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
            constants_str::RUNTIME_FORWARDED_FOR_HEADER_NAME,
            http::HeaderValue::from_static(constants_str::VALUE_203_0_113_7),
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
            constants_str::RUNTIME_FORWARDED_FOR_HEADER_NAME,
            http::HeaderValue::from_static(constants_str::NOT_AN_IP),
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
            constants_str::RUNTIME_FORWARDED_FOR_HEADER_NAME,
            http::HeaderValue::from_static(constants_str::VALUE_203_0_113_1_NOT_AN_IP),
        );
        assert_eq!(
            resolved(&mixed_headers, "10.0.0.10:80", vec![range("10.0.0.0/24")]),
            "10.0.0.10"
        );
        let mut headers = http::HeaderMap::new();
        let _inserted_first = headers.append(
            constants_str::RUNTIME_FORWARDED_FOR_HEADER_NAME,
            http::HeaderValue::from_static(constants_str::VALUE_203_0_113_1),
        );
        let _inserted_second = headers.append(
            constants_str::RUNTIME_FORWARDED_FOR_HEADER_NAME,
            http::HeaderValue::from_static(constants_str::VALUE_203_0_113_2),
        );
        assert_eq!(
            resolved(&headers, "10.0.0.10:80", vec![range("10.0.0.0/24")]),
            "10.0.0.10"
        );
    }
    #[test]
    fn oversized_header_falls_back_without_reflecting_input() {
        let oversized = constants_str::VALUE_1
            .repeat(super::MAX_FORWARDED_HEADER_BYTES.saturating_add(constants_usize::ONE));
        let mut headers = http::HeaderMap::new();
        let _previous = headers.insert(
            constants_str::RUNTIME_FORWARDED_FOR_HEADER_NAME,
            http::HeaderValue::from_str(oversized.as_str()).expect(
                "6353255d oversized_header_falls_back_without_reflecting_input invariant must hold",
            ),
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
