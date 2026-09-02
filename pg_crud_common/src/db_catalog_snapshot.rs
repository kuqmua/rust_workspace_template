#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq)]
pub struct DbCatalogSnapshot {
    objects: crate::db_object_snapshots::DbObjectSnapshots,
}

impl DbCatalogSnapshot {
    #[must_use]
    pub fn new(mut objects: crate::db_object_snapshots::DbObjectSnapshots) -> Self {
        objects.sort();
        Self { objects }
    }
}
