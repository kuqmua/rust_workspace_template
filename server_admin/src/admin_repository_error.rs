#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
pub(crate) enum AdminRepositoryError {
    #[error("stored admin value does not satisfy its contract")]
    InvalidStoredValue,
    #[error("admin repository query failed: {0:?}")]
    Sqlx(crate::sqlx_admin_error::SqlxAdminError),
}

impl From<crate::sqlx_admin_error::SqlxAdminError> for AdminRepositoryError {
    fn from(value: crate::sqlx_admin_error::SqlxAdminError) -> Self {
        Self::Sqlx(value)
    }
}
