#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
use bounded_types::bounded_vec::BoundedVec;

#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, newtype::FromInner)]
pub(super) struct ServiceCatalogEntries(
    pub(super) BoundedVec<crate::service_catalog_entry::ServiceCatalogEntry, 0, { usize::MAX }>,
);
