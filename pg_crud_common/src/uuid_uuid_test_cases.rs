#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    proc_macro_newtype_from_inner::FromInner,
    proc_macro_newtype_into_iterator::IntoIterator,
)]
pub struct UuidUuidTestCases([uuid::Uuid; 1]);
