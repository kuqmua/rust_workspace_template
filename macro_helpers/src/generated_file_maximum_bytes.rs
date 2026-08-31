#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    newtype::FromInner,
    newtype::IntoInnerFrom,
)]
pub(super) struct GeneratedFileMaximumBytes(usize);
