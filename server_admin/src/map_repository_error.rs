pub(in crate::domain_types::auth) fn map_repository_error(
    repository_error: crate::repository::AdminRepositoryError,
) -> super::super::AdminError {
    match repository_error {
        crate::repository::AdminRepositoryError::InvalidStoredValue => {
            super::super::AdminError::Validation
        }
        crate::repository::AdminRepositoryError::Sqlx(sqlx_error) => {
            super::super::AdminError::from(sqlx_error)
        }
    }
}
