#[allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "request service owner modules read this private tracing configuration"
)]
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug)]
#[allow(clippy::arbitrary_source_item_ordering)] // alignment order required by optimal_memory_layout takes precedence over alphabetical field order
pub struct HttpRequestSpanConfig {
    pub(super) service_name: server_observability::service_name::ServiceName,
    pub(super) trusted_proxy_ranges: crate::trusted_proxy_ranges::TrustedProxyRanges,
    pub(super) server_address: crate::client_socket_addr::ClientSocketAddr,
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
}
