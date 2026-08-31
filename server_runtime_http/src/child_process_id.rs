#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    Ord,
    PartialEq,
    PartialOrd,
    newtype::DerefInner,
    newtype::FromInner,
)]
pub struct ChildProcessId(u64);
