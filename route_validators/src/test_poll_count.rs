#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    newtype::DerefInner,
    newtype::DerefMutInner,
    newtype::FromInner,
)]
pub(super) struct TestPollCount(usize);
