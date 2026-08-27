#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
pub(crate) enum AdminRepositoryError {
    #[error("stored admin value does not satisfy its contract")]
    InvalidStoredValue,
    #[error("admin repository query failed: {0:?}")]
    Sqlx(crate::domain_types::SqlxAdminError),
}

impl From<crate::domain_types::SqlxAdminError> for AdminRepositoryError {
    fn from(error: crate::domain_types::SqlxAdminError) -> Self {
        Self::Sqlx(error)
    }
}
