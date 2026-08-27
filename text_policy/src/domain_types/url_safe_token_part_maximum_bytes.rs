#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
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
