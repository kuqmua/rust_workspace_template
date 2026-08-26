pub(in crate::domain_types::auth) fn map_unique_violation<Error>(
    value: Error,
) -> super::super::AdminError
where
    Error: Into<sqlx::Error>,
{
    let error = value.into();
    if pg_crud_common::domain_types::classify_pg_error(
        pg_crud_common::domain_types::SqlxPgErrorRef::from(&error),
    ) == pg_crud_common::domain_types::PgErrorKind::UniqueViolation
    {
        super::super::AdminError::Conflict
    } else {
        super::super::AdminError::from(error)
    }
}
