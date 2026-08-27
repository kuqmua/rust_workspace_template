use super::*;

pub fn validate_postgres_catalog(
    expected: DbCatalogSnapshot,
    observed: DbCatalogSnapshot,
) -> Result<(), DbSchemaConformanceError> {
    validate_snapshot(
        expected,
        observed,
        |expected_snapshot, observed_snapshot| DbSchemaConformanceError::CatalogMismatch {
            expected: expected_snapshot,
            observed: observed_snapshot,
        },
    )
}
