#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "request service owner modules read this private tracing configuration"
)]
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug)]
#[allow(clippy::arbitrary_source_item_ordering)] // alignment order required by optimal_memory_layout takes precedence over alphabetical field order
pub struct HttpRequestSpanConfig {
    pub(super) service_name: super::ServiceName,
    pub(super) trusted_proxy_ranges: super::TrustedProxyRanges,
    pub(super) server_address: super::ClientSocketAddr,
}
impl HttpRequestSpanConfig {
    #[must_use]
    pub const fn new(
        service_name: super::ServiceName,
        server_address: super::ClientSocketAddr,
        trusted_proxy_ranges: super::TrustedProxyRanges,
    ) -> Self {
        Self {
            service_name,
            trusted_proxy_ranges,
            server_address,
        }
    }
}
