#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
pub(crate) enum AdminRepositoryError {
    #[error("{message}", message = constants_str::ADMIN_DIAGNOSTIC_STORED_ADMIN_VALUE_DOES_NOT_SATISFY_ITS_CONTRACT)]
    InvalidStoredValue,
    #[error("{message}: {0:?}", message = constants_str::ADMIN_DIAGNOSTIC_ADMIN_REPOSITORY_QUERY_FAILED)]
    Sqlx(crate::sqlx_admin_error::SqlxAdminError),
}

impl From<crate::sqlx_admin_error::SqlxAdminError> for AdminRepositoryError {
    fn from(value: crate::sqlx_admin_error::SqlxAdminError) -> Self {
        Self::Sqlx(value)
    }
}
