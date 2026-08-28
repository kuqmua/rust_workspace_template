#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(Debug, Clone, Copy, optimal_memory_layout::OptimalMemoryLayout, newtype::FromInner)]
pub(super) struct UriSuffixRef<'suffix_lt>(pub(super) &'suffix_lt str);
