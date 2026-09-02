#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq)]
pub struct DbTableSnapshot {
    columns: crate::db_column_snapshots::DbColumnSnapshots,
    objects: crate::db_object_snapshots::DbObjectSnapshots,
}

impl DbTableSnapshot {
    #[must_use]
    pub fn new(
        mut db_column_snapshots: crate::db_column_snapshots::DbColumnSnapshots,
        mut db_object_snapshots: crate::db_object_snapshots::DbObjectSnapshots,
    ) -> Self {
        db_column_snapshots.sort();
        db_object_snapshots.sort();
        Self {
            columns: db_column_snapshots,
            objects: db_object_snapshots,
        }
    }
}
