#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    newtype::FromInner,
    newtype::IntoInnerFrom,
    newtype::NotInner,
)]
pub(super) struct TestPollLimitReached(bool);
