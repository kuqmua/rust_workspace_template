use crate::SqlxAdminMigrateError;

#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
pub(crate) enum AdminMigrateErrorInner {
    #[error("migration failed: {0:?}")]
    Migration(SqlxAdminMigrateError),
    #[error("permission reconciliation failed: {0:?}")]
    Reconciliation(crate::SqlxAdminError),
}
