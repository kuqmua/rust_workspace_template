#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq)]
pub struct OutboundHostAllowlist(
    bounded_types::bounded_vec::BoundedVec<
        crate::outbound_allowed_host::OutboundAllowedHost,
        1,
        64,
    >,
);

impl TryFrom<Vec<crate::outbound_allowed_host::OutboundAllowedHost>> for OutboundHostAllowlist {
    type Error = crate::outbound_host_allowlist_error::OutboundHostAllowlistError;

    fn try_from(
        mut value: Vec<crate::outbound_allowed_host::OutboundAllowedHost>,
    ) -> Result<Self, Self::Error> {
        value.sort();
        value.dedup();
        bounded_types::bounded_vec::BoundedVec::try_from(value)
            .map(Self)
            .map_err(|error| match error {
                bounded_types::bounded_value_error::BoundedValueError::BelowMin { .. } => {
                    crate::outbound_host_allowlist_error::OutboundHostAllowlistError::Empty
                }
                bounded_types::bounded_value_error::BoundedValueError::AboveMax { .. }
                | bounded_types::bounded_value_error::BoundedValueError::InvalidBounds { .. } => {
                    crate::outbound_host_allowlist_error::OutboundHostAllowlistError::TooManyHosts
                }
            })
    }
}

impl OutboundHostAllowlist {
    pub fn validate(
        &self,
        reqwest_outbound_url: &crate::reqwest_outbound_url::ReqwestOutboundUrl,
    ) -> Result<(), crate::outbound_host_allowlist_error::OutboundHostAllowlistError> {
        let host = reqwest_outbound_url
            .host_str()
            .ok_or(crate::outbound_host_allowlist_error::OutboundHostAllowlistError::InvalidHost)?;
        if self
            .0
            .binary_search_by(|allowed| allowed.as_str().cmp(host))
            .is_ok()
        {
            Ok(())
        } else {
            Err(crate::outbound_host_allowlist_error::OutboundHostAllowlistError::HostNotAllowed)
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_allowlist_limits_unique_hosts_and_sorts_before_lookup() {
        assert!([(0usize, 1usize), (1usize, 1usize), (64usize, 1usize), (65usize, 1usize), (64usize, 2usize), (1usize, 100usize)]
            .into_iter().all(|(count, repetitions)| {
                (0usize..count).map(|index| crate::outbound_allowed_host::OutboundAllowedHost::try_from(format!("{}{index}", constants_str::X)))
                    .collect::<Result<Vec<_>, _>>().is_ok_and(|hosts| {
                        let repeated = hosts.into_iter().rev().flat_map(|host| std::iter::repeat_n(host, repetitions)).collect::<Vec<_>>();
                        let result = crate::outbound_host_allowlist::OutboundHostAllowlist::try_from(repeated);
                        if count == 0usize {
                            result == Err(crate::outbound_host_allowlist_error::OutboundHostAllowlistError::Empty)
                        } else if count > 64usize {
                            result == Err(crate::outbound_host_allowlist_error::OutboundHostAllowlistError::TooManyHosts)
                        } else {
                            result.is_ok_and(|allowlist| allowlist.0.len().get() == count
                                && allowlist.0.windows(2usize).all(|pair| matches!(pair, [first, second] if first < second))
                                && allowlist.0.iter().all(|host| {
                                    let text = format!("{}{}", constants_str::HTTPS_SCHEME_PREFIX, host.as_str());
                                    reqwest::Url::parse(text.as_str()).is_ok_and(|url| {
                                        allowlist.validate(&crate::reqwest_outbound_url::ReqwestOutboundUrl::from(url)) == Ok(())
                                    })
                                }))
                        }
                    })
            }));
    }
    #[test]
    fn test_allowlist_matches_canonical_ipv6_independently_of_port_and_rejects_hostless_urls() {
        let canonical = format!("[{}]", std::net::Ipv6Addr::LOCALHOST);
        let allowed_result =
            crate::outbound_allowed_host::OutboundAllowedHost::try_from(canonical.clone());
        assert!(allowed_result.is_ok());
        let Ok(allowed) = allowed_result else {
            return;
        };
        let allowlist_result =
            crate::outbound_host_allowlist::OutboundHostAllowlist::try_from(vec![allowed]);
        assert!(allowlist_result.is_ok());
        let Ok(allowlist) = allowlist_result else {
            return;
        };
        assert!([80u16, 443u16, 65535u16].into_iter().all(|port| {
            let text = format!(
                "{}{canonical}:{port}/{}",
                constants_str::HTTPS_SCHEME_PREFIX,
                constants_str::X
            );
            reqwest::Url::parse(text.as_str()).is_ok_and(|url| {
                allowlist.validate(&crate::reqwest_outbound_url::ReqwestOutboundUrl::from(url))
                    == Ok(())
            })
        }));
        let text = format!("{}:{}", constants_str::X, constants_str::X);
        assert!(reqwest::Url::parse(text.as_str()).is_ok_and(|url| {
            allowlist.validate(&crate::reqwest_outbound_url::ReqwestOutboundUrl::from(url))
                == Err(
                    crate::outbound_host_allowlist_error::OutboundHostAllowlistError::InvalidHost,
                )
        }));
        let other = format!(
            "{}[{}]",
            constants_str::HTTPS_SCHEME_PREFIX,
            std::net::Ipv6Addr::UNSPECIFIED
        );
        assert!(reqwest::Url::parse(other.as_str()).is_ok_and(|url| {
            allowlist.validate(&crate::reqwest_outbound_url::ReqwestOutboundUrl::from(url))
                == Err(crate::outbound_host_allowlist_error::OutboundHostAllowlistError::HostNotAllowed)
        }));
    }
}
