#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout)]
#[must_use]
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    proc_macro_newtype_foundation::FromInner,
    proc_macro_newtype_foundation::GetInner,
)]
#[accessor(pub(super))]
pub struct PartIndex(usize);
