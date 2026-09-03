pub(super) const URL_SAFE_TOKEN_PART_MAXIMUM_BYTES: usize = 4096usize;

#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    proc_macro_newtype_from_inner::FromInner,
    proc_macro_newtype_into_inner_from::IntoInnerFrom,
)]
pub struct UrlSafeTokenPartMaximumBytes(usize);
