#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, newtype::FromInner)]
pub struct OutboundIpAddr(std::net::IpAddr);

impl OutboundIpAddr {
    pub(crate) const fn get(self) -> std::net::IpAddr {
        self.0
    }
}
