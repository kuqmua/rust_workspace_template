pub(crate) trait SnapshotMismatch: PartialEq + Sized {
    fn mismatch(
        expected: Self,
        observed: Self,
    ) -> crate::db_schema_conformance_error::DbSchemaConformanceError;
}
