pub(crate) trait SnapshotMismatch: PartialEq + Sized {
    fn mismatch(
        expected: Self,
        observed: Self,
    ) -> crate::db_schema_conformance_error::DbSchemaConformanceError;
}

impl SnapshotMismatch for crate::db_catalog_snapshot::DbCatalogSnapshot {
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

impl SnapshotMismatch for crate::db_table_snapshot::DbTableSnapshot {
    fn mismatch(
        expected: Self,
        observed: Self,
    ) -> crate::db_schema_conformance_error::DbSchemaConformanceError {
        crate::db_schema_conformance_error::DbSchemaConformanceError::Mismatch {
            expected,
            observed,
        }
    }
}
