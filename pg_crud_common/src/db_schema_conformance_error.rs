#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
pub enum DbSchemaConformanceError {
    #[error("PostgreSQL catalog differs from the expected snapshot")]
    CatalogMismatch {
        expected: crate::db_catalog_snapshot::DbCatalogSnapshot,
        observed: crate::db_catalog_snapshot::DbCatalogSnapshot,
    },
    #[error("PostgreSQL columns differ from the generated table descriptor")]
    ColumnContractMismatch {
        expected: crate::db_column_contract_snapshots::DbColumnContractSnapshots,
        observed: crate::db_column_contract_snapshots::DbColumnContractSnapshots,
    },
    #[error("PostgreSQL exact defaults differ from the reviewed table contract")]
    DefaultContractMismatch {
        expected: crate::db_object_snapshots::DbObjectSnapshots,
        observed: crate::db_object_snapshots::DbObjectSnapshots,
    },
    #[error("generated CRUD configuration refers to a column absent from its table descriptor")]
    DescriptorFieldMismatch(crate::db_schema_text::DbSchemaText),
    #[error("PostgreSQL CHECK/index objects differ from the reviewed table contract")]
    ExtendedObjectContractMismatch {
        expected: crate::db_object_snapshots::DbObjectSnapshots,
        observed: crate::db_object_snapshots::DbObjectSnapshots,
    },
    #[error("failed to inspect PostgreSQL schema: {0}")]
    Inspection(crate::sqlx_db_schema_inspection_error::SqlxDbSchemaInspectionError),
    #[error("PostgreSQL key constraints differ from the generated table descriptor")]
    KeyContractMismatch {
        expected: crate::db_key_contract_snapshots::DbKeyContractSnapshots,
        observed: crate::db_key_contract_snapshots::DbKeyContractSnapshots,
    },
    #[error("PostgreSQL table schema differs from the expected snapshot")]
    Mismatch {
        expected: crate::db_table_snapshot::DbTableSnapshot,
        observed: crate::db_table_snapshot::DbTableSnapshot,
    },
    #[error("PostgreSQL schema text exceeds the supported limit")]
    SchemaTextTooLong(crate::db_schema_text::DbSchemaTextTryFromStringError),
    #[error("PostgreSQL returned an unsupported catalog object kind")]
    UnknownObjectKind,
}
