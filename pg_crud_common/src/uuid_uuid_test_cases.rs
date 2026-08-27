#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    optimal_memory_layout::OptimalMemoryLayout,
    newtype::FromInner,
    newtype::IntoIterator,
)]
pub struct UuidUuidTestCases([uuid::Uuid; 1]);
