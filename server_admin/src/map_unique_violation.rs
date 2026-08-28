pub(crate) fn map_unique_violation<Error>(value: Error) -> crate::AdminError
where
    Error: Into<sqlx::Error>,
{
    let error = value.into();
    if pg_crud_common::domain_types::classify_pg_error(
        pg_crud_common::domain_types::SqlxPgErrorRef::from(&error),
    ) == pg_crud_common::domain_types::PgErrorKind::UniqueViolation
    {
        crate::AdminError::Conflict
    } else {
        crate::AdminError::from(error)
    }
}
