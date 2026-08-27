#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    PartialOrd,
    newtype::FromInner,
)]
pub struct AdminAuditDetailsBytes(pub(super) usize);
