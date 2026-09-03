#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    PartialOrd,
    Ord,
    proc_macro_newtype::Display,
    proc_macro_newtype::FromInner,
    proc_macro_newtype::GetInner,
)]
#[allow(
    clippy::module_name_repetitions,
    reason = "lint suppression is required here"
)]
pub struct PgBoundedVecLen(usize);
