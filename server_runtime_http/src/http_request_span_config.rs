#[derive(proc_macro_getters::Getters, proc_macro_new::New)]
#[getters(bare)]
#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Clone, Debug)]
#[allow(
    clippy::arbitrary_source_item_ordering,
    reason = "http request span config keeps declaration order aligned with generated layout or processing flow"
)]
pub struct HttpRequestSpanConfig {
    service_name: server_observability::service_name::ServiceName,
    #[constructor(order = 2)]
    trusted_proxy_ranges: crate::trusted_proxy_ranges::TrustedProxyRanges,
    #[getters(copy)]
    #[constructor(order = 1)]
    server_address: crate::client_socket_addr::ClientSocketAddr,
}
