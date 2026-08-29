pub(crate) fn map_unique_violation<Error>(value: Error) -> crate::admin_error::AdminError
where
    Error: Into<sqlx::Error>,
{
    let error = value.into();
    if pg_crud_common::classify_pg_error::classify_pg_error(
        pg_crud_common::sqlx_pg_error_ref::SqlxPgErrorRef::from(&error),
    ) == pg_crud_common::pg_error_kind::PgErrorKind::UniqueViolation
    {
        crate::admin_error::AdminError::Conflict
    } else {
        crate::admin_error::AdminError::from(error)
    }
}
