#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq)]
pub struct TrustedProxyRange {
    network: super::IpnetNetwork,
}

impl TryFrom<String> for TrustedProxyRange {
    type Error = super::TrustedProxyRangeParseError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let Some((address_text, prefix_text)) = value.split_once('/') else {
            return Err(super::TrustedProxyRangeParseError::MissingPrefix);
        };
        let network_address = address_text.parse::<std::net::IpAddr>().map_err(|source| {
            super::TrustedProxyRangeParseError::InvalidAddress {
                source: super::ClientAddrParseError::from(source),
            }
        })?;
        let prefix_bits = prefix_text.parse::<u8>().map_err(|source| {
            super::TrustedProxyRangeParseError::InvalidPrefix {
                source: super::ParseIntError::from(source),
            }
        })?;
        let Ok(network) = ipnet::IpNet::new(network_address, prefix_bits) else {
            return Err(super::TrustedProxyRangeParseError::PrefixExceedsAddressWidth);
        };
        Ok(Self {
            network: super::IpnetNetwork::from(network),
        })
    }
}

impl TrustedProxyRange {
    pub(super) fn contains(self, candidate_ip: super::ParsedIpAddr) -> super::StdRangeContains {
        super::StdRangeContains::from(self.network.0.contains(&candidate_ip.0))
    }
}
