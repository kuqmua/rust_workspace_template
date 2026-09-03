#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    proc_macro_newtype_from_inner::FromInner,
    proc_macro_newtype_deref_inner::DerefInner,
    proc_macro_newtype_into_inner::IntoInner,
    proc_macro_getters::Getters,
)]
pub(crate) struct AxumAdminQuery<Value>(Value);
