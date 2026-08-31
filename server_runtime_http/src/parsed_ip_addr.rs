#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    newtype::FromInner,
)]
pub(super) struct ParsedIpAddr(std::net::IpAddr);

impl ParsedIpAddr {
    pub(crate) const fn get(self) -> std::net::IpAddr {
        self.0
    }
}
