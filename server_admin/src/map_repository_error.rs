pub(crate) fn map_repository_error(
    repository_error: crate::admin_repository_error::AdminRepositoryError,
) -> crate::admin_error::AdminError {
    match repository_error {
        crate::admin_repository_error::AdminRepositoryError::InvalidStoredValue => {
            crate::admin_error::AdminError::Validation
        }
        crate::admin_repository_error::AdminRepositoryError::Sqlx(sqlx_error) => {
            crate::admin_error::AdminError::from(sqlx_error)
        }
    }
}
