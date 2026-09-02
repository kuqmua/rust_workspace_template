#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    proc_macro_newtype::FromInner,
    proc_macro_newtype::IntoInnerFrom,
    proc_macro_getters::Getters,
)]
pub(crate) struct StdAdminRateLimitCount(i64);
