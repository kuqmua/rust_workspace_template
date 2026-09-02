#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    proc_macro_newtype::FromInner,
)]
pub struct OutboundIpAddr(std::net::IpAddr);

impl OutboundIpAddr {
    pub(crate) const fn get(self) -> std::net::IpAddr {
        self.0
    }
}
