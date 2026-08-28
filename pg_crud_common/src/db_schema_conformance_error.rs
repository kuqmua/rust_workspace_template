#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
pub enum DbSchemaConformanceError {
    #[error("PostgreSQL catalog differs from the expected snapshot")]
    CatalogMismatch {
        expected: super::DbCatalogSnapshot,
        observed: super::DbCatalogSnapshot,
    },
    #[error("PostgreSQL columns differ from the generated table descriptor")]
    ColumnContractMismatch {
        expected: super::DbColumnContractSnapshots,
        observed: super::DbColumnContractSnapshots,
    },
    #[error("PostgreSQL exact defaults differ from the reviewed table contract")]
    DefaultContractMismatch {
        expected: super::DbObjectSnapshots,
        observed: super::DbObjectSnapshots,
    },
    #[error("generated CRUD configuration refers to a column absent from its table descriptor")]
    DescriptorFieldMismatch(super::DbSchemaText),
    #[error("PostgreSQL CHECK/index objects differ from the reviewed table contract")]
    ExtendedObjectContractMismatch {
        expected: super::DbObjectSnapshots,
        observed: super::DbObjectSnapshots,
    },
    #[error("failed to inspect PostgreSQL schema: {0}")]
    Inspection(super::SqlxDbSchemaInspectionError),
    #[error("PostgreSQL key constraints differ from the generated table descriptor")]
    KeyContractMismatch {
        expected: super::DbKeyContractSnapshots,
        observed: super::DbKeyContractSnapshots,
    },
    #[error("PostgreSQL table schema differs from the expected snapshot")]
    Mismatch {
        expected: super::DbTableSnapshot,
        observed: super::DbTableSnapshot,
    },
    #[error("PostgreSQL schema text exceeds the supported limit")]
    SchemaTextTooLong(super::DbSchemaTextTryFromStringError),
    #[error("PostgreSQL returned an unsupported catalog object kind")]
    UnknownObjectKind,
}
