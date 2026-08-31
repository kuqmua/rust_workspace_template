#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    newtype::FromInner,
    newtype::IntoInnerFrom,
)]
pub(super) struct CommandIdx(usize);
