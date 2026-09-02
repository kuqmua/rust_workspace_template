#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq)]
pub struct DbCatalogSnapshot {
    objects: crate::db_object_snapshots::DbObjectSnapshots,
}

impl DbCatalogSnapshot {
    #[must_use]
    pub fn new(mut db_object_snapshots: crate::db_object_snapshots::DbObjectSnapshots) -> Self {
        db_object_snapshots.sort();
        Self {
            objects: db_object_snapshots,
        }
    }
}
