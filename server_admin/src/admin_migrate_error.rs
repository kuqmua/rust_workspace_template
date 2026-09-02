#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
pub enum AdminMigrateError {
    #[error("failed to prepare administrator schema: migration failed: {0:?}")]
    Migration(crate::sqlx_admin_migrate_error::SqlxAdminMigrateError),
    #[error("failed to prepare administrator schema: permission reconciliation failed: {0:?}")]
    Reconciliation(crate::sqlx_admin_error::SqlxAdminError),
}
impl From<crate::sqlx_admin_migrate_error::SqlxAdminMigrateError> for AdminMigrateError {
    fn from(
        sqlx_admin_migrate_error: crate::sqlx_admin_migrate_error::SqlxAdminMigrateError,
    ) -> Self {
        Self::Migration(sqlx_admin_migrate_error)
    }
}
impl From<crate::sqlx_admin_error::SqlxAdminError> for AdminMigrateError {
    fn from(sqlx_admin_error: crate::sqlx_admin_error::SqlxAdminError) -> Self {
        Self::Reconciliation(sqlx_admin_error)
    }
}
