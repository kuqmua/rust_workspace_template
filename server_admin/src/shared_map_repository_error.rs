pub(in crate::domain_types::auth) fn map_repository_error(
    repository_error: crate::adapters::repository::AdminRepositoryError,
) -> super::super::AdminError {
    match repository_error {
        crate::adapters::repository::AdminRepositoryError::InvalidStoredValue => {
            super::super::AdminError::Validation
        }
        crate::adapters::repository::AdminRepositoryError::Sqlx(sqlx_error) => {
            super::super::AdminError::from(sqlx_error)
        }
    }
}
