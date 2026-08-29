use super::{AdminMigrateErrorInner, SqlxAdminMigrateError};

#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
#[error("failed to prepare administrator schema: {0}")]
#[derive(newtype::FromInner)]
pub struct AdminMigrateError(AdminMigrateErrorInner);
impl From<SqlxAdminMigrateError> for AdminMigrateError {
    fn from(error: SqlxAdminMigrateError) -> Self {
        Self(AdminMigrateErrorInner::Migration(error))
    }
}
impl From<crate::SqlxAdminError> for AdminMigrateError {
    fn from(error: crate::SqlxAdminError) -> Self {
        Self(AdminMigrateErrorInner::Reconciliation(error))
    }
}
