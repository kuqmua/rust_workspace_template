#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, newtype::FromInner)]
pub struct OutboundIpAddr(pub(super) std::net::IpAddr);
