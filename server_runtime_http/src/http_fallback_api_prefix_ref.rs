#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    proc_macro_newtype::FromInner,
    proc_macro_newtype::GetInner,
)]
#[accessor(pub(crate))]
pub struct HttpFallbackApiPrefixRef<'value_lt>(&'value_lt str);
