pub(crate) fn map_repository_error(
    repository_error: crate::repository::AdminRepositoryError,
) -> crate::AdminError {
    match repository_error {
        crate::repository::AdminRepositoryError::InvalidStoredValue => {
            crate::AdminError::Validation
        }
        crate::repository::AdminRepositoryError::Sqlx(sqlx_error) => {
            crate::AdminError::from(sqlx_error)
        }
    }
}
