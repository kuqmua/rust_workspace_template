#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq,
)]
pub struct TrustedProxyRange {
    network: crate::ipnet_network::IpnetNetwork,
}

impl TryFrom<String> for TrustedProxyRange {
    type Error = crate::trusted_proxy_range_parse_error::TrustedProxyRangeParseError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let Some((address_text, prefix_text)) = value.split_once('/') else {
            return Err(
                crate::trusted_proxy_range_parse_error::TrustedProxyRangeParseError::MissingPrefix,
            );
        };
        let network_address = address_text.parse::<std::net::IpAddr>().map_err(|source| {
            crate::trusted_proxy_range_parse_error::TrustedProxyRangeParseError::InvalidAddress {
                source: crate::client_addr_parse_error::ClientAddrParseError::from(source),
            }
        })?;
        let prefix_bits = prefix_text.parse::<u8>().map_err(|source| {
            crate::trusted_proxy_range_parse_error::TrustedProxyRangeParseError::InvalidPrefix {
                source: crate::parse_int_error::ParseIntError::from(source),
            }
        })?;
        let Ok(network) = ipnet::IpNet::new(network_address, prefix_bits) else {
            return Err(crate::trusted_proxy_range_parse_error::TrustedProxyRangeParseError::PrefixExceedsAddressWidth);
        };
        Ok(Self {
            network: crate::ipnet_network::IpnetNetwork::from(network),
        })
    }
}

impl TrustedProxyRange {
    pub(super) fn contains(
        self,
        parsed_ip_addr: crate::parsed_ip_addr::ParsedIpAddr,
    ) -> crate::std_range_contains::StdRangeContains {
        let address = parsed_ip_addr.get();
        let network = self.network.get();
        crate::std_range_contains::StdRangeContains::from(
            network.contains(&address)
                || match address {
                    std::net::IpAddr::V6(ipv6_address) => {
                        ipv6_address.to_ipv4_mapped().is_some_and(|ipv4_address| {
                            network.contains(&std::net::IpAddr::V4(ipv4_address))
                        })
                    }
                    std::net::IpAddr::V4(ipv4_address) => match network {
                        ipnet::IpNet::V6(ipv6_network)
                            if ipv6_network.prefix_len() >= 96u8
                                && ipv6_network.addr().to_ipv4_mapped().is_some() =>
                        {
                            ipv6_network.contains(&ipv4_address.to_ipv6_mapped())
                        }
                        ipnet::IpNet::V4(_) | ipnet::IpNet::V6(_) => false,
                    },
                },
        )
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_proxy_range_prefix_boundaries_control_same_family_membership() {
        assert!(
            [
                (
                    std::net::IpAddr::V4(std::net::Ipv4Addr::LOCALHOST),
                    std::net::IpAddr::V4(std::net::Ipv4Addr::UNSPECIFIED),
                    32u8
                ),
                (
                    std::net::IpAddr::V6(std::net::Ipv6Addr::LOCALHOST),
                    std::net::IpAddr::V6(std::net::Ipv6Addr::UNSPECIFIED),
                    128u8
                ),
            ]
            .into_iter()
            .all(|(address, other_address, width)| {
                [0u8, width].into_iter().all(|prefix| {
                    crate::trusted_proxy_range::TrustedProxyRange::try_from(format!(
                        "{address}/{prefix}"
                    ))
                    .is_ok_and(|range| {
                        range
                            .contains(crate::parsed_ip_addr::ParsedIpAddr::from(address))
                            .get()
                            && range
                                .contains(crate::parsed_ip_addr::ParsedIpAddr::from(other_address))
                                .get()
                                == (prefix == 0u8)
                    })
                })
            })
        );
    }

    #[test]
    fn test_proxy_range_parser_distinguishes_invalid_prefix_syntax_from_address_width() {
        let address = std::net::Ipv4Addr::LOCALHOST;
        assert!(matches!(
            crate::trusted_proxy_range::TrustedProxyRange::try_from(address.to_string()),
            Err(crate::trusted_proxy_range_parse_error::TrustedProxyRangeParseError::MissingPrefix)
        ));
        assert!([String::new(), constants_str::X.to_owned(), 256u16.to_string(), '/'.to_string()]
            .into_iter().all(|prefix| {
                let text = format!("{address}/{prefix}");
                matches!(crate::trusted_proxy_range::TrustedProxyRange::try_from(text), Err(crate::trusted_proxy_range_parse_error::TrustedProxyRangeParseError::InvalidPrefix { .. }))
            }));
        assert!([
            format!("{}/{}", std::net::Ipv4Addr::LOCALHOST, 33u8),
            format!("{}/{}", std::net::Ipv6Addr::LOCALHOST, 129u8),
        ].into_iter().all(|text| {
            matches!(crate::trusted_proxy_range::TrustedProxyRange::try_from(text), Err(crate::trusted_proxy_range_parse_error::TrustedProxyRangeParseError::PrefixExceedsAddressWidth))
        }));
        let invalid_address = format!("{}/{}", constants_str::X, 24u8);
        assert!(matches!(crate::trusted_proxy_range::TrustedProxyRange::try_from(invalid_address), Err(crate::trusted_proxy_range_parse_error::TrustedProxyRangeParseError::InvalidAddress { .. })));
    }
}
