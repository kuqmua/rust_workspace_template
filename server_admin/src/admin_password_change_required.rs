#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    serde::Serialize,
    proc_macro_newtype_deref_inner::DerefInner,
    proc_macro_newtype_from_inner::FromInner,
)]
#[serde(transparent)]
#[derive(proc_macro_getters::Getters)]
pub(crate) struct AdminPasswordChangeRequired(bool);
