#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error, newtype::FromInner,
)]
#[error(transparent)]
pub struct AdminGeneratedTablesValidationError(
    pg_crud_common::domain_types::DbSchemaConformanceError,
);
