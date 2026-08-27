pub(in crate::domain_types::outbound_url) fn outbound_address_disposition(
    address: super::super::OutboundIpAddr,
) -> super::OutboundAddressDisposition {
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
                outbound_address_disposition(super::super::OutboundIpAddr::from(
                    std::net::IpAddr::V4(mapped),
                )) == super::OutboundAddressDisposition::Forbidden
            },
        ),
    };
    if forbidden {
        super::OutboundAddressDisposition::Forbidden
    } else {
        super::OutboundAddressDisposition::Allowed
    }
}
