#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
use super::ServiceCatalogEntry;
use bounded_types::domain_types::vector::BoundedVec;

#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, newtype::FromInner)]
pub(super) struct ServiceCatalogEntries(
    pub(super) BoundedVec<ServiceCatalogEntry, 0, { usize::MAX }>,
);
