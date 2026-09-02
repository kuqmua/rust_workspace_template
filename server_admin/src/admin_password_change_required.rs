#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    serde::Serialize,
    proc_macro_newtype::DerefInner,
    proc_macro_newtype::FromInner,
)]
#[serde(transparent)]
#[derive(proc_macro_getters::Getters)]
pub(crate) struct AdminPasswordChangeRequired(bool);
