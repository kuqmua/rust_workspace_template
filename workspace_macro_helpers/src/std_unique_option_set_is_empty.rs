#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    proc_macro_newtype_foundation::FromInner,
    proc_macro_newtype_foundation::GetInner,
)]
pub struct StdUniqueOptionSetIsEmpty(bool);
