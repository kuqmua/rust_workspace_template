#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    newtype::AsRefStr,
    newtype::FromInner,
)]
pub(in crate::domain_types::start) struct AdminCsrApiUrlSuffixRef<'suffix_lt>(&'suffix_lt str);
