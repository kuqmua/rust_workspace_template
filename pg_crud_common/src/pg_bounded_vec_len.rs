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
    reason = "pg bounded vec len requires this localized allowance for generated or framework-constrained code verified by focused tests"
)]
pub struct PgBoundedVecLen(usize);
