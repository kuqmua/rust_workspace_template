#[must_use]
pub fn classify_pg_error(
    error_ref: crate::domain_types::SqlxPgErrorRef<'_>,
) -> crate::domain_types::PgErrorKind {
    match error_ref.get() {
        sqlx::Error::Database(database_error) => database_error.code().as_deref().map_or(
            crate::domain_types::PgErrorKind::Unknown,
            |code| match code {
                constants_str::PG_SQLSTATE_STRING_DATA_RIGHT_TRUNCATION => {
                    crate::domain_types::PgErrorKind::StringDataRightTruncation
                }
                constants_str::PG_SQLSTATE_NUMERIC_VALUE_OUT_OF_RANGE => {
                    crate::domain_types::PgErrorKind::NumericValueOutOfRange
                }
                constants_str::PG_SQLSTATE_INVALID_TEXT_REPRESENTATION => {
                    crate::domain_types::PgErrorKind::InvalidTextRepresentation
                }
                constants_str::PG_SQLSTATE_NOT_NULL_VIOLATION => {
                    crate::domain_types::PgErrorKind::NotNullViolation
                }
                constants_str::PG_SQLSTATE_FOREIGN_KEY_VIOLATION => {
                    crate::domain_types::PgErrorKind::ForeignKeyViolation
                }
                constants_str::PG_SQLSTATE_UNIQUE_VIOLATION => {
                    crate::domain_types::PgErrorKind::UniqueViolation
                }
                constants_str::PG_SQLSTATE_CHECK_VIOLATION => {
                    crate::domain_types::PgErrorKind::CheckViolation
                }
                constants_str::PG_SQLSTATE_SERIALIZATION_FAILURE => {
                    crate::domain_types::PgErrorKind::SerializationFailure
                }
                constants_str::PG_SQLSTATE_DEADLOCK_DETECTED => {
                    crate::domain_types::PgErrorKind::Deadlock
                }
                _ => crate::domain_types::PgErrorKind::Unknown,
            },
        ),
        sqlx::Error::PoolTimedOut => crate::domain_types::PgErrorKind::PoolTimedOut,
        sqlx::Error::Io(_) | sqlx::Error::Tls(_) | sqlx::Error::PoolClosed => {
            crate::domain_types::PgErrorKind::Connection
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
        | sqlx::Error::WorkerCrashed => crate::domain_types::PgErrorKind::Unknown,
        _ => std::convert::identity(crate::domain_types::PgErrorKind::Unknown),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn classifies_non_database_errors() {
        assert_eq!(
            super::classify_pg_error(crate::domain_types::SqlxPgErrorRef::from(
                &sqlx::Error::PoolTimedOut
            )),
            crate::domain_types::PgErrorKind::PoolTimedOut
        );
        assert_eq!(
            super::classify_pg_error(crate::domain_types::SqlxPgErrorRef::from(
                &sqlx::Error::PoolClosed
            )),
            crate::domain_types::PgErrorKind::Connection
        );
        assert_eq!(
            super::classify_pg_error(crate::domain_types::SqlxPgErrorRef::from(
                &sqlx::Error::RowNotFound
            )),
            crate::domain_types::PgErrorKind::Unknown
        );
    }
}
