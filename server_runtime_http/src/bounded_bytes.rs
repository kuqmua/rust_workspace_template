#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Eq,
    PartialEq,
    newtype::FromInner,
    newtype::IntoInner,
)]
pub struct BoundedBytes(Vec<u8>);
