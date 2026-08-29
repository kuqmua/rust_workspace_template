#[must_use]
pub fn classify_pg_error(
    error_ref: crate::sqlx_pg_error_ref::SqlxPgErrorRef<'_>,
) -> crate::pg_error_kind::PgErrorKind {
    match error_ref.get() {
        sqlx::Error::Database(database_error) => database_error.code().as_deref().map_or(
            crate::pg_error_kind::PgErrorKind::Unknown,
            |code| match code {
                constants_str::test_fixtures::PG_SQLSTATE_STRING_DATA_RIGHT_TRUNCATION => {
                    crate::pg_error_kind::PgErrorKind::StringDataRightTruncation
                }
                constants_str::test_fixtures::PG_SQLSTATE_NUMERIC_VALUE_OUT_OF_RANGE => {
                    crate::pg_error_kind::PgErrorKind::NumericValueOutOfRange
                }
                constants_str::test_fixtures::PG_SQLSTATE_INVALID_TEXT_REPRESENTATION => {
                    crate::pg_error_kind::PgErrorKind::InvalidTextRepresentation
                }
                constants_str::test_fixtures::PG_SQLSTATE_NOT_NULL_VIOLATION => {
                    crate::pg_error_kind::PgErrorKind::NotNullViolation
                }
                constants_str::test_fixtures::PG_SQLSTATE_FOREIGN_KEY_VIOLATION => {
                    crate::pg_error_kind::PgErrorKind::ForeignKeyViolation
                }
                constants_str::test_fixtures::PG_SQLSTATE_UNIQUE_VIOLATION => {
                    crate::pg_error_kind::PgErrorKind::UniqueViolation
                }
                constants_str::test_fixtures::PG_SQLSTATE_CHECK_VIOLATION => {
                    crate::pg_error_kind::PgErrorKind::CheckViolation
                }
                constants_str::test_fixtures::PG_SQLSTATE_SERIALIZATION_FAILURE => {
                    crate::pg_error_kind::PgErrorKind::SerializationFailure
                }
                constants_str::test_fixtures::PG_SQLSTATE_DEADLOCK_DETECTED => {
                    crate::pg_error_kind::PgErrorKind::Deadlock
                }
                _ => crate::pg_error_kind::PgErrorKind::Unknown,
            },
        ),
        sqlx::Error::PoolTimedOut => crate::pg_error_kind::PgErrorKind::PoolTimedOut,
        sqlx::Error::Io(_) | sqlx::Error::Tls(_) | sqlx::Error::PoolClosed => {
            crate::pg_error_kind::PgErrorKind::Connection
        }
        sqlx::Error::AnyDriverError(_)
        | sqlx::Error::BeginFailed
        | sqlx::Error::ColumnDecode { .. }
        | sqlx::Error::ColumnIndexOutOfBounds { .. }
        | sqlx::Error::ColumnNotFound(_)
        | sqlx::Error::Configuration(_)
        | sqlx::Error::Decode(_)
        | sqlx::Error::Encode(_)
        | sqlx::Error::InvalidArgument(_)
        | sqlx::Error::InvalidSavePointStatement
        | sqlx::Error::Migrate(_)
        | sqlx::Error::Protocol(_)
        | sqlx::Error::RowNotFound
        | sqlx::Error::TypeNotFound { .. }
        | sqlx::Error::WorkerCrashed => crate::pg_error_kind::PgErrorKind::Unknown,
        _ => std::convert::identity(crate::pg_error_kind::PgErrorKind::Unknown),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn classifies_non_database_errors() {
        assert_eq!(
            crate::classify_pg_error::classify_pg_error(
                crate::sqlx_pg_error_ref::SqlxPgErrorRef::from(&sqlx::Error::PoolTimedOut)
            ),
            crate::pg_error_kind::PgErrorKind::PoolTimedOut
        );
        assert_eq!(
            crate::classify_pg_error::classify_pg_error(
                crate::sqlx_pg_error_ref::SqlxPgErrorRef::from(&sqlx::Error::PoolClosed)
            ),
            crate::pg_error_kind::PgErrorKind::Connection
        );
        assert_eq!(
            crate::classify_pg_error::classify_pg_error(
                crate::sqlx_pg_error_ref::SqlxPgErrorRef::from(&sqlx::Error::RowNotFound)
            ),
            crate::pg_error_kind::PgErrorKind::Unknown
        );
    }
}
