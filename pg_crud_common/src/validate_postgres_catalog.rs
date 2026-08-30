pub fn validate_postgres_catalog(
    expected: crate::db_catalog_snapshot::DbCatalogSnapshot,
    observed: crate::db_catalog_snapshot::DbCatalogSnapshot,
) -> Result<(), crate::db_schema_conformance_error::DbSchemaConformanceError> {
    crate::validate_snapshot::validate_snapshot(expected, observed)
}
