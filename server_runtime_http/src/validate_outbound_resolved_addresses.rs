pub(crate) fn validate_outbound_resolved_addresses(
    outbound_host_policy: crate::outbound_host_policy::OutboundHostPolicy,
    mut addresses: impl Iterator<Item = crate::outbound_ip_addr::OutboundIpAddr>,
) -> Result<(), crate::outbound_url_error::OutboundUrlError> {
    let Some(first) = addresses.next() else {
        return Err(crate::outbound_url_error::OutboundUrlError::MissingResolvedAddress);
    };
    if outbound_host_policy == crate::outbound_host_policy::OutboundHostPolicy::RejectPrivate
        && std::iter::once(first).chain(addresses).any(|address| {
            crate::resolve_outbound_address_disposition::resolve_outbound_address_disposition(
                address,
            ) == crate::outbound_address_disposition::OutboundAddressDisposition::Forbidden
        })
    {
        Err(crate::outbound_url_error::OutboundUrlError::ForbiddenHost)
    } else {
        Ok(())
    }
}
