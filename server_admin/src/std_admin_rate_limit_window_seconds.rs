#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    proc_macro_newtype_from_inner::FromInner,
    proc_macro_newtype_into_inner_from::IntoInnerFrom,
    proc_macro_getters::Getters,
)]
pub(crate) struct StdAdminRateLimitWindowSeconds(i32);
