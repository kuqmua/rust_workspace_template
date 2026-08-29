#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
#[error("failed to prepare administrator schema: {0}")]
#[derive(newtype::FromInner)]
pub struct AdminMigrateError(crate::admin_migrate_error_inner::AdminMigrateErrorInner);
impl From<crate::sqlx_admin_migrate_error::SqlxAdminMigrateError> for AdminMigrateError {
    fn from(error: crate::sqlx_admin_migrate_error::SqlxAdminMigrateError) -> Self {
        Self(crate::admin_migrate_error_inner::AdminMigrateErrorInner::Migration(error))
    }
}
impl From<crate::sqlx_admin_error::SqlxAdminError> for AdminMigrateError {
    fn from(error: crate::sqlx_admin_error::SqlxAdminError) -> Self {
        Self(crate::admin_migrate_error_inner::AdminMigrateErrorInner::Reconciliation(error))
    }
}
