#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug)]
#[allow(clippy::arbitrary_source_item_ordering)] // alignment order required by optimal_memory_layout takes precedence over alphabetical field order
pub struct HttpRequestSpanConfig {
    service_name: server_observability::service_name::ServiceName,
    trusted_proxy_ranges: crate::trusted_proxy_ranges::TrustedProxyRanges,
    server_address: crate::client_socket_addr::ClientSocketAddr,
}
impl HttpRequestSpanConfig {
    #[must_use]
    pub const fn new(
        service_name: server_observability::service_name::ServiceName,
        server_address: crate::client_socket_addr::ClientSocketAddr,
        trusted_proxy_ranges: crate::trusted_proxy_ranges::TrustedProxyRanges,
    ) -> Self {
        Self {
            service_name,
            trusted_proxy_ranges,
            server_address,
        }
    }

    pub(crate) const fn server_address(&self) -> crate::client_socket_addr::ClientSocketAddr {
        self.server_address
    }

    pub(crate) const fn service_name(&self) -> &server_observability::service_name::ServiceName {
        &self.service_name
    }

    pub(crate) const fn trusted_proxy_ranges(
        &self,
    ) -> &crate::trusted_proxy_ranges::TrustedProxyRanges {
        &self.trusted_proxy_ranges
    }
}
