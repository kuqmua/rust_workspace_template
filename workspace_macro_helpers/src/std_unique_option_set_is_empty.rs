#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    proc_macro_newtype_foundation_foundation_from_inner::FromInner,
    proc_macro_newtype_foundation_foundation_get_inner::GetInner,
)]
pub struct StdUniqueOptionSetIsEmpty(bool);
