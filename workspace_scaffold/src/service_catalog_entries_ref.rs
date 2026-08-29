#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, newtype::FromInner)]
pub(super) struct ServiceCatalogEntriesRef<'entries_lt>(
    pub(super) &'entries_lt [crate::service_catalog_entry::ServiceCatalogEntry],
);
