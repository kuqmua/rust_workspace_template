#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    Ord,
    PartialEq,
    PartialOrd,
    proc_macro_newtype_from_inner::FromInner,
)]
pub struct DbColumnNullable(bool);
