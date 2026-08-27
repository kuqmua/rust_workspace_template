#[derive(optimal_memory_layout::OptimalMemoryLayout, newtype::FromInner, newtype::NotInner)]
pub(super) struct TestPollLimitReached(pub(super) bool);
