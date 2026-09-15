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
impl crate::snapshot_mismatch::SnapshotMismatch for DbCatalogSnapshot {
    fn mismatch(
        expected: Self,
        observed: Self,
    ) -> crate::db_schema_conformance_error::DbSchemaConformanceError {
        crate::db_schema_conformance_error::DbSchemaConformanceError::CatalogMismatch {
            expected,
            observed,
        }
    }
}
