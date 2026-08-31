#[must_use]
pub fn resolve_client_ip(
    headers: crate::http_header_map_ref::HttpHeaderMapRef<'_>,
    peer: crate::client_socket_addr::ClientSocketAddr,
    trusted_proxy_ranges: &crate::trusted_proxy_ranges::TrustedProxyRanges,
) -> crate::resolved_client_ip_addr::ResolvedClientIpAddr {
    let peer_ip = peer.ip();
    if !trusted_proxy_ranges
        .contains(crate::parsed_ip_addr::ParsedIpAddr::from(peer_ip))
        .get()
    {
        return crate::resolved_client_ip_addr::ResolvedClientIpAddr::from(peer_ip);
    }
    let parsed_forwarded_ip = || {
        let values = headers
            .get()
            .get_all(constants_str::RUNTIME_FORWARDED_FOR_HEADER_NAME);
        let mut iter = values.iter();
        let value = iter.next()?;
        if iter.next().is_some() || value.as_bytes().len() > constants_usize::VALUE_4_096 {
            return None;
        }
        let value_text = value.to_str().ok()?;
        let (count, first, rightmost_untrusted) = value_text.split(',').map(str::trim).try_fold(
            (constants_usize::ZERO, None, None),
            |(count, first, rightmost_untrusted), entry| {
                if count >= constants_usize::VALUE_32 {
                    return None;
                }
                let parsed = entry.parse::<std::net::IpAddr>().ok()?;
                let next_first = first.or(Some(parsed));
                let next_rightmost_untrusted = if trusted_proxy_ranges
                    .contains(crate::parsed_ip_addr::ParsedIpAddr::from(parsed))
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
            .get()
            .get_all(constants_str::RUNTIME_REAL_IP_HEADER_NAME);
        let mut iter = values.iter();
        let value = iter.next()?;
        if iter.next().is_some() || value.as_bytes().len() > constants_usize::VALUE_4_096 {
            return None;
        }
        value.to_str().ok()?.trim().parse::<std::net::IpAddr>().ok()
    };
    crate::resolved_client_ip_addr::ResolvedClientIpAddr::from(
        parsed_forwarded_ip()
            .or_else(parsed_real_ip)
            .unwrap_or(peer_ip),
    )
}
