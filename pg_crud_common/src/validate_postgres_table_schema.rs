use crate::*;

pub fn validate_postgres_table_schema(
    expected: DbTableSnapshot,
    observed: DbTableSnapshot,
) -> Result<(), DbSchemaConformanceError> {
    validate_snapshot(
        expected,
        observed,
        |expected_snapshot, observed_snapshot| DbSchemaConformanceError::Mismatch {
            expected: expected_snapshot,
            observed: observed_snapshot,
        },
    )
}
