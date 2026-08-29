#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
pub(crate) enum AdminMigrateErrorInner {
    #[error("migration failed: {0:?}")]
    Migration(crate::sqlx_admin_migrate_error::SqlxAdminMigrateError),
    #[error("permission reconciliation failed: {0:?}")]
    Reconciliation(crate::sqlx_admin_error::SqlxAdminError),
}
