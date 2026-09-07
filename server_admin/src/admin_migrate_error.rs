#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
pub enum AdminMigrateError {
    #[error("{message}: {0}", message = constants_str::ADMIN_IDEMPOTENCY_PREPARATION_FAILED)]
    Idempotency(#[source] pg_table::sqlx_pg_table_idempotency_error::SqlxPgTableIdempotencyError),
    #[error("{message}: {0:?}", message = constants_str::ADMIN_DIAGNOSTIC_FAILED_TO_PREPARE_ADMINISTRATOR_SCHEMA_MIGRATION_FAILED)]
    Migration(crate::sqlx_admin_migrate_error::SqlxAdminMigrateError),
    #[error("{message}: {0:?}", message = constants_str::ADMIN_DIAGNOSTIC_FAILED_TO_PREPARE_ADMINISTRATOR_SCHEMA_PERMISSION_RECONCILIATION_FAILED)]
    Reconciliation(crate::sqlx_admin_error::SqlxAdminError),
}
impl From<crate::sqlx_admin_migrate_error::SqlxAdminMigrateError> for AdminMigrateError {
    fn from(value: crate::sqlx_admin_migrate_error::SqlxAdminMigrateError) -> Self {
        Self::Migration(value)
    }
}
impl From<crate::sqlx_admin_error::SqlxAdminError> for AdminMigrateError {
    fn from(value: crate::sqlx_admin_error::SqlxAdminError) -> Self {
        Self::Reconciliation(value)
    }
}
