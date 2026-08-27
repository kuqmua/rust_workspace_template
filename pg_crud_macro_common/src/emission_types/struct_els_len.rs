#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    newtype::FromInner,
    newtype::GetInner,
)]
pub struct StructElsLen(usize);
