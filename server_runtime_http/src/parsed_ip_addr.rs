#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    proc_macro_newtype::FromInner,
)]
pub(super) struct ParsedIpAddr(std::net::IpAddr);

impl ParsedIpAddr {
    pub(crate) const fn get(self) -> std::net::IpAddr {
        self.0
    }
}
