#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Default,
    Eq,
    PartialEq,
    newtype::IntoInnerFrom,
    newtype::FromInner,
)]
pub struct DevelopmentIdentityCount(pub(super) usize);
