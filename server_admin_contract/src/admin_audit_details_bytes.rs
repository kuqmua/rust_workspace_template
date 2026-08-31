#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    PartialOrd,
    newtype::FromInner,
    newtype::GetInner,
)]
pub struct AdminAuditDetailsBytes(usize);
