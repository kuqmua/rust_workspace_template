#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
use bounded_types::bounded_vec::BoundedVec;

#[derive(optimal_memory_layout::OptimalMemoryLayout, newtype::FromInner, newtype::IntoIterator)]
pub(crate) struct InitEntries(
    pub(super) BoundedVec<crate::initialization_entry::InitializationEntry, 0, { usize::MAX }>,
);
