#[must_use]
pub(crate) fn resolve_outbound_address_disposition(
    outbound_ip_addr: crate::outbound_ip_addr::OutboundIpAddr,
) -> crate::outbound_address_disposition::OutboundAddressDisposition {
    let forbidden = match outbound_ip_addr.get() {
        std::net::IpAddr::V4(ipv4_address) => {
            let octets = ipv4_address.octets();
            let [first_octet, second_octet, third_octet, _fourth_octet] = octets;
            ipv4_address.is_broadcast()
                || ipv4_address.is_link_local()
                || ipv4_address.is_loopback()
                || ipv4_address.is_multicast()
                || ipv4_address.is_private()
                || ipv4_address.is_unspecified()
                || first_octet == constants_u8::ZERO
                || (first_octet == 100u8 && (64u8..=127u8).contains(&second_octet))
                || (first_octet == 192u8
                    && second_octet == constants_u8::ZERO
                    && third_octet == constants_u8::ZERO)
                || (first_octet == 192u8
                    && second_octet == constants_u8::ZERO
                    && third_octet == 2u8)
                || (first_octet == 198u8 && (second_octet == 18u8 || second_octet == 19u8))
                || (first_octet == 198u8 && second_octet == 51u8 && third_octet == 100u8)
                || (first_octet == 203u8
                    && second_octet == constants_u8::ZERO
                    && third_octet == 113u8)
                || first_octet >= 240u8
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
                resolve_outbound_address_disposition(crate::outbound_ip_addr::OutboundIpAddr::from(
                    std::net::IpAddr::V4(mapped),
                )) == crate::outbound_address_disposition::OutboundAddressDisposition::Forbidden
            },
        ),
    };
    if forbidden {
        crate::outbound_address_disposition::OutboundAddressDisposition::Forbidden
    } else {
        crate::outbound_address_disposition::OutboundAddressDisposition::Allowed
    }
}
