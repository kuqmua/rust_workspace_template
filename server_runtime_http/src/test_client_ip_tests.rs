#![allow(
    clippy::arbitrary_source_item_ordering,
    reason = "owner modules and related behavior retain their intentional facade ordering"
)]
#[cfg(test)]
mod tests {
    fn range(str: &str) -> crate::trusted_proxy_range::TrustedProxyRange {
        crate::trusted_proxy_range::TrustedProxyRange::try_from(str.to_owned())
            .expect(constants_str::DIAGNOSTIC_46CC9E0A)
    }
    fn resolved(
        header_map: &http::HeaderMap,
        str: &str,
        vec: Vec<crate::trusted_proxy_range::TrustedProxyRange>,
    ) -> String {
        crate::resolve_client_ip::resolve_client_ip(
            crate::http_header_map_ref::HttpHeaderMapRef::from(header_map),
            crate::client_socket_addr::ClientSocketAddr::from(
                str.parse::<std::net::SocketAddr>()
                    .expect(constants_str::DIAGNOSTIC_262819A8),
            ),
            &crate::trusted_proxy_ranges::TrustedProxyRanges::try_from(vec)
                .expect(constants_str::DIAGNOSTIC_38546D0B),
        )
        .to_string()
    }
    #[test]
    fn test_proxy_ranges_match_ipv4_mapped_addresses() {
        let trusted_range = range(constants_str::VALUE_127_0_0_1_32);
        let mapped = std::net::Ipv4Addr::LOCALHOST.to_ipv6_mapped();
        assert!(
            trusted_range
                .contains(crate::parsed_ip_addr::ParsedIpAddr::from(
                    std::net::IpAddr::V6(mapped),
                ))
                .get()
        );
        assert!(
            !trusted_range
                .contains(crate::parsed_ip_addr::ParsedIpAddr::from(
                    std::net::IpAddr::V6(
                        std::net::Ipv4Addr::new(127u8, 0u8, 0u8, 2u8).to_ipv6_mapped()
                    ),
                ))
                .get()
        );
        assert!(
            !trusted_range
                .contains(crate::parsed_ip_addr::ParsedIpAddr::from(
                    std::net::IpAddr::V6(std::net::Ipv6Addr::LOCALHOST),
                ))
                .get()
        );
        let mut mapped_network_text = mapped.to_string();
        mapped_network_text.push('/');
        mapped_network_text.push_str(constants_str::VALUE_128);
        assert!(
            range(mapped_network_text.as_str())
                .contains(crate::parsed_ip_addr::ParsedIpAddr::from(
                    std::net::IpAddr::V6(mapped),
                ))
                .get()
        );
    }

    #[test]
    fn test_forwarded_ip_is_resolved_for_ipv4_mapped_proxy() {
        let mut headers = http::HeaderMap::new();
        let _previous = headers.insert(
            constants_str::RUNTIME_FORWARDED_FOR_HEADER_NAME,
            http::HeaderValue::from_static(constants_str::VALUE_203_0_113_1),
        );
        let ranges = crate::trusted_proxy_ranges::TrustedProxyRanges::try_from(vec![range(
            constants_str::VALUE_127_0_0_1_32,
        )]);
        assert!(matches!(ranges, Ok(trusted_ranges) if {
            let resolve = |client_socket_addr: crate::client_socket_addr::ClientSocketAddr| {
                crate::resolve_client_ip::resolve_client_ip(
                    crate::http_header_map_ref::HttpHeaderMapRef::from(&headers),
                    client_socket_addr,
                    &trusted_ranges,
                )
            };
            let trusted_peer = std::net::Ipv4Addr::LOCALHOST.to_ipv6_mapped();
            let untrusted_peer = std::net::Ipv4Addr::new(127u8, 0u8, 0u8, 2u8).to_ipv6_mapped();
            resolve(crate::client_socket_addr::ClientSocketAddr::from(
                std::net::SocketAddr::from((trusted_peer, 0u16)),
            )) == crate::resolved_client_ip_addr::ResolvedClientIpAddr::from(
                std::net::IpAddr::V4(std::net::Ipv4Addr::new(203u8, 0u8, 113u8, 1u8)),
            ) && resolve(crate::client_socket_addr::ClientSocketAddr::from(
                std::net::SocketAddr::from((untrusted_peer, 0u16)),
            )) == crate::resolved_client_ip_addr::ResolvedClientIpAddr::from(
                std::net::IpAddr::V6(untrusted_peer),
            )
        }));
    }

    #[test]
    fn test_trusted_proxy_range_parser_stops_after_exceeding_item_limit() {
        let mut text = vec![constants_str::VALUE_127_0_0_1_32; constants_usize::VALUE_128]
            .join(constants_str::COMMA_SPACE);
        assert!(matches!(
            crate::parse_trusted_proxy_ranges::parse_trusted_proxy_ranges(
                crate::trusted_proxy_ranges_text_ref::TrustedProxyRangesTextRef::from(
                    text.as_str()
                )
            )
            .map(|ranges| ranges
                .contains(crate::parsed_ip_addr::ParsedIpAddr::from(
                    std::net::IpAddr::V4(std::net::Ipv4Addr::LOCALHOST)
                ))
                .get()),
            Ok(true)
        ));
        text.push_str(constants_str::COMMA_SPACE);
        text.push_str(constants_str::VALUE_127_0_0_1_32);
        text.push_str(constants_str::COMMA_SPACE);
        text.push_str(constants_str::VALUE_127_0_0_1);
        assert!(matches!(
            crate::parse_trusted_proxy_ranges::parse_trusted_proxy_ranges(
                crate::trusted_proxy_ranges_text_ref::TrustedProxyRangesTextRef::from(
                    text.as_str()
                )
            ),
            Err(
                crate::trusted_proxy_ranges_parse_error::TrustedProxyRangesParseError::Ranges(
                    crate::trusted_proxy_ranges_error::TrustedProxyRangesError::TooMany
                )
            )
        ));
    }

    #[test]
    fn test_trusted_proxy_ranges_reject_oversized_lists() {
        let item = range(constants_str::VALUE_127_0_0_1_32);
        let values = vec![item; constants_usize::VALUE_128.saturating_add(constants_usize::ONE)];
        assert_eq!(
            crate::trusted_proxy_ranges::TrustedProxyRanges::try_from(values),
            Err(crate::trusted_proxy_ranges_error::TrustedProxyRangesError::TooMany)
        );
    }
    #[test]
    fn test_trusted_proxy_ranges_text_parses_comma_separated_ranges() {
        let ranges = crate::parse_trusted_proxy_ranges::parse_trusted_proxy_ranges(
            crate::trusted_proxy_ranges_text_ref::TrustedProxyRangesTextRef::from(
                constants_str::VALUE_127_0_0_1_32_PATH_1_128,
            ),
        )
        .expect(constants_str::DIAGNOSTIC_60AD1A64);
        assert_eq!(
            crate::resolve_client_ip::resolve_client_ip(
                crate::http_header_map_ref::HttpHeaderMapRef::from(&http::HeaderMap::new()),
                crate::client_socket_addr::ClientSocketAddr::from(
                    constants_str::VALUE_127_0_0_1_8080
                        .parse::<std::net::SocketAddr>()
                        .expect(constants_str::DIAGNOSTIC_A6F1A8F9)
                ),
                &ranges,
            )
            .to_string(),
            constants_str::VALUE_127_0_0_1
        );
    }
    #[test]
    fn test_trusted_proxy_ranges_text_rejects_empty_list_entries() {
        assert!(matches!(
            crate::parse_trusted_proxy_ranges::parse_trusted_proxy_ranges(crate::trusted_proxy_ranges_text_ref::TrustedProxyRangesTextRef::from(
                constants_str::VALUE_396A5D02,
            )),
            Err(crate::trusted_proxy_ranges_parse_error::TrustedProxyRangesParseError::Range(
                crate::trusted_proxy_range_parse_error::TrustedProxyRangeParseError::MissingPrefix
            ))
        ));
        let empty = crate::parse_trusted_proxy_ranges::parse_trusted_proxy_ranges(
            crate::trusted_proxy_ranges_text_ref::TrustedProxyRangesTextRef::from(
                constants_str::SPACE,
            ),
        )
        .expect(constants_str::DIAGNOSTIC_639128BA);
        assert_eq!(
            empty,
            crate::trusted_proxy_ranges::TrustedProxyRanges::default()
        );
    }
    #[test]
    fn test_untrusted_peer_cannot_spoof_forwarded_header() {
        let mut headers = http::HeaderMap::new();
        let _previous = headers.insert(
            constants_str::RUNTIME_FORWARDED_FOR_HEADER_NAME,
            http::HeaderValue::from_static(constants_str::VALUE_203_0_113_1),
        );
        assert_eq!(
            resolved(&headers, constants_str::VALUE_BA3E0E24, Vec::new()),
            constants_str::VALUE_8C9BBD8A
        );
    }
    #[test]
    fn test_trusted_chain_resolves_first_untrusted_address_from_right() {
        let mut headers = http::HeaderMap::new();
        let _previous = headers.insert(
            constants_str::RUNTIME_FORWARDED_FOR_HEADER_NAME,
            http::HeaderValue::from_static(constants_str::VALUE_203_0_113_7_10_0_0_8_10_0_0),
        );
        assert_eq!(
            resolved(
                &headers,
                constants_str::VALUE_52553922,
                vec![range(constants_str::VALUE_A34D80F7)]
            ),
            constants_str::VALUE_203_0_113_7
        );
    }
    #[test]
    fn test_ipv4_range_does_not_trust_ipv6_peer() {
        let mut headers = http::HeaderMap::new();
        let _previous = headers.insert(
            constants_str::RUNTIME_FORWARDED_FOR_HEADER_NAME,
            http::HeaderValue::from_static(constants_str::VALUE_203_0_113_7),
        );
        assert_eq!(
            resolved(
                &headers,
                constants_str::VALUE_09F22780,
                vec![range(constants_str::VALUE_4A7EA159)]
            ),
            constants_str::PATH_1
        );
    }
    #[test]
    fn test_malformed_and_multiple_headers_fall_back_to_peer() {
        let mut malformed_headers = http::HeaderMap::new();
        let _inserted_malformed = malformed_headers.append(
            constants_str::RUNTIME_FORWARDED_FOR_HEADER_NAME,
            http::HeaderValue::from_static(constants_str::NOT_AN_IP),
        );
        assert_eq!(
            resolved(
                &malformed_headers,
                constants_str::VALUE_52553922,
                vec![range(constants_str::VALUE_A34D80F7)]
            ),
            constants_str::VALUE_EBB856CA
        );
        let mut mixed_headers = http::HeaderMap::new();
        let _inserted_mixed = mixed_headers.append(
            constants_str::RUNTIME_FORWARDED_FOR_HEADER_NAME,
            http::HeaderValue::from_static(constants_str::VALUE_203_0_113_1_NOT_AN_IP),
        );
        assert_eq!(
            resolved(
                &mixed_headers,
                constants_str::VALUE_52553922,
                vec![range(constants_str::VALUE_A34D80F7)]
            ),
            constants_str::VALUE_EBB856CA
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
            resolved(
                &headers,
                constants_str::VALUE_52553922,
                vec![range(constants_str::VALUE_A34D80F7)]
            ),
            constants_str::VALUE_EBB856CA
        );
    }
    #[test]
    fn test_oversized_header_falls_back_without_reflecting_input() {
        let oversized = constants_str::VALUE_1
            .repeat(constants_usize::VALUE_4_096.saturating_add(constants_usize::ONE));
        let mut headers = http::HeaderMap::new();
        let _previous = headers.insert(
            constants_str::RUNTIME_FORWARDED_FOR_HEADER_NAME,
            http::HeaderValue::from_str(oversized.as_str())
                .expect(constants_str::DIAGNOSTIC_6353255D),
        );
        assert_eq!(
            resolved(
                &headers,
                constants_str::VALUE_52553922,
                vec![range(constants_str::VALUE_A34D80F7)]
            ),
            constants_str::VALUE_EBB856CA
        );
    }
    #[test]
    fn test_prefixes_are_validated() {
        assert!(matches!(
            crate::trusted_proxy_range::TrustedProxyRange::try_from(constants_str::VALUE_9604A2A6.to_owned()),
            Err(crate::trusted_proxy_range_parse_error::TrustedProxyRangeParseError::PrefixExceedsAddressWidth)
        ));
        assert!(matches!(
            crate::trusted_proxy_range::TrustedProxyRange::try_from(constants_str::VALUE_CB3D2AAD.to_owned()),
            Err(crate::trusted_proxy_range_parse_error::TrustedProxyRangeParseError::PrefixExceedsAddressWidth)
        ));
    }
}
