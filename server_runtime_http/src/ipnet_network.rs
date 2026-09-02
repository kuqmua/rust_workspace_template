#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    proc_macro_newtype::FromInner,
    proc_macro_newtype::GetInner,
)]
#[accessor(pub(crate))]
pub(super) struct IpnetNetwork(ipnet::IpNet);
