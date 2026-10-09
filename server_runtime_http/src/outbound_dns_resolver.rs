#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, proc_macro_new::New,
)]
pub(crate) struct OutboundDnsResolver {
    host_policy: crate::outbound_host_policy::OutboundHostPolicy,
}

impl reqwest::dns::Resolve for OutboundDnsResolver {
    fn resolve(&self, name: reqwest::dns::Name) -> reqwest::dns::Resolving {
        let host = name.as_str().to_owned();
        let host_policy = self.host_policy;
        Box::pin(async move {
            let addresses = tokio::net::lookup_host((host.as_str(), 0u16))
                .await?
                .collect::<Vec<_>>();
            crate::validate_outbound_resolved_addresses::validate_outbound_resolved_addresses(
                host_policy,
                addresses
                    .iter()
                    .map(std::net::SocketAddr::ip)
                    .map(crate::outbound_ip_addr::OutboundIpAddr::from),
            )?;
            let resolved: reqwest::dns::Addrs = Box::new(addresses.into_iter());
            Ok(resolved)
        })
    }
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    #[ignore = "exercises the native DNS adapter with numeric loopback; no external DNS required"]
    async fn test_outbound_dns_numeric_loopback_preserves_addresses_and_policy_error() {
        let name_result = std::net::Ipv4Addr::LOCALHOST
            .to_string()
            .parse::<reqwest::dns::Name>();
        let Ok(name) = name_result else {
            assert!(
                name_result.is_ok_and(|reqwest_dns_name| !reqwest_dns_name.as_str().is_empty())
            );
            return;
        };
        let resolver = crate::outbound_dns_resolver::OutboundDnsResolver::new(
            crate::outbound_host_policy::OutboundHostPolicy::AllowPrivate,
        );
        let addresses_result = reqwest::dns::Resolve::resolve(&resolver, name).await;
        assert!(
            addresses_result.is_ok_and(|addresses| addresses.eq(std::iter::once(
                std::net::SocketAddr::from((std::net::Ipv4Addr::LOCALHOST, 0u16)),
            )))
        );
        let rejected_name_result = std::net::Ipv4Addr::LOCALHOST
            .to_string()
            .parse::<reqwest::dns::Name>();
        let Ok(rejected_name) = rejected_name_result else {
            assert!(
                rejected_name_result
                    .is_ok_and(|reqwest_dns_name| !reqwest_dns_name.as_str().is_empty())
            );
            return;
        };
        let rejecting_resolver = crate::outbound_dns_resolver::OutboundDnsResolver::new(
            crate::outbound_host_policy::OutboundHostPolicy::RejectPrivate,
        );
        let result = reqwest::dns::Resolve::resolve(&rejecting_resolver, rejected_name).await;
        assert!(result.is_err_and(|source| {
            source.downcast_ref::<crate::outbound_url_error::OutboundUrlError>()
                == Some(&crate::outbound_url_error::OutboundUrlError::ForbiddenHost)
        }));
    }

    #[tokio::test]
    #[ignore = "exercises native DNS input rejection without external DNS"]
    async fn test_outbound_dns_preserves_local_lookup_input_error() {
        let name_result = char::from(0u8).to_string().parse::<reqwest::dns::Name>();
        let Ok(name) = name_result else {
            assert!(
                name_result.is_ok_and(|reqwest_dns_name| !reqwest_dns_name.as_str().is_empty())
            );
            return;
        };
        let resolver = crate::outbound_dns_resolver::OutboundDnsResolver::new(
            crate::outbound_host_policy::OutboundHostPolicy::AllowPrivate,
        );
        let result = reqwest::dns::Resolve::resolve(&resolver, name).await;
        assert!(result.is_err_and(|source| {
            source
                .downcast_ref::<std::io::Error>()
                .is_some_and(|io_error| io_error.kind() == std::io::ErrorKind::InvalidInput)
        }));
    }
}
