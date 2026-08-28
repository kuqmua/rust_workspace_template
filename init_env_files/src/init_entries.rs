#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
use crate::domain_types::InitializationEntry;
use bounded_types::BoundedVec;

#[derive(optimal_memory_layout::OptimalMemoryLayout, newtype::FromInner, newtype::IntoIterator)]
pub(crate) struct InitEntries(pub(super) BoundedVec<InitializationEntry, 0, { usize::MAX }>);
