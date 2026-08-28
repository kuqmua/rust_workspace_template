#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    newtype::AsRefOwned,
    newtype::Display,
    newtype::FromInner,
)]
pub struct ResolvedClientIpAddr(std::net::IpAddr);
