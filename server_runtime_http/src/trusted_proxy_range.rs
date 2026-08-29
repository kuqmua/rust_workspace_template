#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq)]
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
        candidate_ip: crate::parsed_ip_addr::ParsedIpAddr,
    ) -> crate::std_range_contains::StdRangeContains {
        crate::std_range_contains::StdRangeContains::from(self.network.0.contains(&candidate_ip.0))
    }
}
