#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq)]
pub struct DbTableSnapshot {
    pub(super) columns: crate::db_column_snapshots::DbColumnSnapshots,
    pub(super) objects: crate::db_object_snapshots::DbObjectSnapshots,
}

impl DbTableSnapshot {
    #[must_use]
    pub fn new(
        mut columns: crate::db_column_snapshots::DbColumnSnapshots,
        mut objects: crate::db_object_snapshots::DbObjectSnapshots,
    ) -> Self {
        columns.sort_unstable();
        objects.sort_unstable();
        Self { columns, objects }
    }
}
