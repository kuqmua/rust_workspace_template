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
