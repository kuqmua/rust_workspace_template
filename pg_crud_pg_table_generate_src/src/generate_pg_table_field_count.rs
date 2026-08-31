#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    newtype::DerefInner,
    newtype::FromInner,
    newtype::IntoInnerFrom,
)]
pub struct GeneratePgTableFieldCount(usize);
