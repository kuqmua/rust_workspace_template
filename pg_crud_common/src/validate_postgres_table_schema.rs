pub fn validate_postgres_table_schema(
    expected: crate::db_table_snapshot::DbTableSnapshot,
    observed: crate::db_table_snapshot::DbTableSnapshot,
) -> Result<(), crate::db_schema_conformance_error::DbSchemaConformanceError> {
    crate::validate_snapshot::validate_snapshot(
        expected,
        observed,
        |expected_snapshot, observed_snapshot| {
            crate::db_schema_conformance_error::DbSchemaConformanceError::Mismatch {
                expected: expected_snapshot,
                observed: observed_snapshot,
            }
        },
    )
}
