#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    proc_macro_newtype::FromInner,
)]
pub(super) struct IpnetNetwork(ipnet::IpNet);

impl IpnetNetwork {
    pub(crate) const fn get(self) -> ipnet::IpNet {
        self.0
    }
}
