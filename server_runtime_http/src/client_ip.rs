#![allow(
    clippy::arbitrary_source_item_ordering,
    reason = "owner modules and related behavior retain their intentional facade ordering"
)]

#[path = "client_ip/client_addr_parse_error.rs"]
mod client_addr_parse_error;
#[path = "client_ip/client_socket_addr.rs"]
mod client_socket_addr;
#[path = "client_ip/http_header_map_ref.rs"]
mod http_header_map_ref;
#[path = "client_ip/ipnet_network.rs"]
mod ipnet_network;
#[path = "client_ip/parse_int_error.rs"]
mod parse_int_error;
#[path = "client_ip/parse_trusted_proxy_ranges.rs"]
mod parse_trusted_proxy_ranges;
#[path = "client_ip/parsed_ip_addr.rs"]
mod parsed_ip_addr;
#[path = "client_ip/resolve_client_ip.rs"]
mod resolve_client_ip;
#[path = "client_ip/resolve_header_text.rs"]
mod resolve_header_text;
#[path = "client_ip/resolved_client_ip_addr.rs"]
mod resolved_client_ip_addr;
#[path = "client_ip/std_range_contains.rs"]
mod std_range_contains;
#[path = "client_ip/trusted_proxy_range.rs"]
mod trusted_proxy_range;
#[path = "client_ip/trusted_proxy_range_parse_error.rs"]
mod trusted_proxy_range_parse_error;
#[path = "client_ip/trusted_proxy_ranges.rs"]
mod trusted_proxy_ranges;
#[path = "client_ip/trusted_proxy_ranges_error.rs"]
mod trusted_proxy_ranges_error;
#[path = "client_ip/trusted_proxy_ranges_parse_error.rs"]
mod trusted_proxy_ranges_parse_error;
#[path = "client_ip/trusted_proxy_ranges_text_ref.rs"]
mod trusted_proxy_ranges_text_ref;

pub use client_addr_parse_error::ClientAddrParseError;
pub use client_socket_addr::ClientSocketAddr;
pub use http_header_map_ref::HttpHeaderMapRef;
use ipnet_network::IpnetNetwork;
pub use parse_int_error::ParseIntError;
pub use parse_trusted_proxy_ranges::parse_trusted_proxy_ranges;
use parsed_ip_addr::ParsedIpAddr;
pub use resolve_client_ip::resolve_client_ip;
pub use resolve_header_text::resolve_header_text;
pub use resolved_client_ip_addr::ResolvedClientIpAddr;
use std_range_contains::StdRangeContains;
pub use trusted_proxy_range::TrustedProxyRange;
pub use trusted_proxy_range_parse_error::TrustedProxyRangeParseError;
pub use trusted_proxy_ranges::TrustedProxyRanges;
pub use trusted_proxy_ranges_error::TrustedProxyRangesError;
pub use trusted_proxy_ranges_parse_error::TrustedProxyRangesParseError;
pub use trusted_proxy_ranges_text_ref::TrustedProxyRangesTextRef;

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
        let values = vec![item; constants_usize::VALUE_128.saturating_add(constants_usize::ONE)];
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
        let empty = super::parse_trusted_proxy_ranges(super::TrustedProxyRangesTextRef::from(
            constants_str::SPACE,
        ))
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
            .repeat(constants_usize::VALUE_4_096.saturating_add(constants_usize::ONE));
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
