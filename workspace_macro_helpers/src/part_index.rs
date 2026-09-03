#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout)]
#[must_use]
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    proc_macro_newtype_foundation_foundation_from_inner::FromInner,
    proc_macro_newtype_foundation_foundation_get_inner::GetInner,
)]
#[accessor(pub(super))]
pub struct PartIndex(usize);
