#[derive(
    Debug,
    Clone,
    Copy,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    proc_macro_newtype::DerefInner,
    proc_macro_newtype::FromInner,
)]
pub(super) struct UriSuffixRef<'suffix_lt>(&'suffix_lt str);
