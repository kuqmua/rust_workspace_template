#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(Debug, optimal_memory_layout::OptimalMemoryLayout, newtype::FromInner)]
pub(super) struct GitCommitLinkOutputRefMut<'output_lt>(pub(super) &'output_lt mut String);
