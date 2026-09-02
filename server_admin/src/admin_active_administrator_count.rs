#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    proc_macro_newtype::FromInner,
    proc_macro_getters::Getters,
)]
pub(crate) struct AdminActiveAdministratorCount(i64);
