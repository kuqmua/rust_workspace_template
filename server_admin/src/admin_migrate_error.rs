#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
pub enum AdminMigrateError {
    #[error("failed to prepare administrator schema: migration failed: {0:?}")]
    Migration(crate::sqlx_admin_migrate_error::SqlxAdminMigrateError),
    #[error("failed to prepare administrator schema: permission reconciliation failed: {0:?}")]
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
