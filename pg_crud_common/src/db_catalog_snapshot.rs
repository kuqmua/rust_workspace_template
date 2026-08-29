#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq)]
pub struct DbCatalogSnapshot {
    pub(super) objects: crate::db_object_snapshots::DbObjectSnapshots,
}

impl DbCatalogSnapshot {
    #[must_use]
    pub fn new(mut objects: crate::db_object_snapshots::DbObjectSnapshots) -> Self {
        objects.sort_unstable();
        Self { objects }
    }
}
