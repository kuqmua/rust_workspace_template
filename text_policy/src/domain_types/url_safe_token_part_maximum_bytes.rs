pub(super) const URL_SAFE_TOKEN_PART_MAXIMUM_BYTES: usize = 4096usize;

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    newtype::FromInner,
)]
pub struct UrlSafeTokenPartMaximumBytes(pub(super) usize);
