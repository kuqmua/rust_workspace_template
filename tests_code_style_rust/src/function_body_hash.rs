#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    proc_macro_newtype_from_inner::FromInner,
)]
pub(super) struct FunctionBodyHash(u64);
