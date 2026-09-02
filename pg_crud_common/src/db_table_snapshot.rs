#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq)]
pub struct DbTableSnapshot {
    columns: crate::db_column_snapshots::DbColumnSnapshots,
    objects: crate::db_object_snapshots::DbObjectSnapshots,
}

impl DbTableSnapshot {
    #[must_use]
    pub fn new(
        mut columns: crate::db_column_snapshots::DbColumnSnapshots,
        mut objects: crate::db_object_snapshots::DbObjectSnapshots,
    ) -> Self {
        columns.sort();
        objects.sort();
        Self { columns, objects }
    }
}
