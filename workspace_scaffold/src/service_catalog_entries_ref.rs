#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
use super::ServiceCatalogEntry;

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, newtype::FromInner)]
pub(super) struct ServiceCatalogEntriesRef<'entries_lt>(
    pub(super) &'entries_lt [ServiceCatalogEntry],
);
