#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    newtype::AsRefStr,
    newtype::FromInner,
)]
pub(crate) struct AdminCsrApiUrlSuffixRef<'suffix_lt>(&'suffix_lt str);
