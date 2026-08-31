#[derive(
    Debug,
    Clone,
    Copy,
    optimal_memory_layout::OptimalMemoryLayout,
    newtype::DerefInner,
    newtype::FromInner,
)]
pub(super) struct UriSuffixRef<'suffix_lt>(&'suffix_lt str);
