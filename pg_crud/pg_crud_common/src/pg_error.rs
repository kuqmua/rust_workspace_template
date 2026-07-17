#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PgErrorKind {
    CheckViolation,
    Connection,
    Deadlock,
    ForeignKeyViolation,
    InvalidTextRepresentation,
    NotNullViolation,
    NumericValueOutOfRange,
    PoolTimedOut,
    SerializationFailure,
    StringDataRightTruncation,
    UniqueViolation,
    Unknown,
}

#[derive(Clone, Copy, Debug, newtype::FromInner)]
pub struct SqlxPgErrorRef<'error_lt>(&'error_lt sqlx::Error);

#[must_use]
pub fn classify_pg_error(error_ref: SqlxPgErrorRef<'_>) -> PgErrorKind {
    let error = error_ref.0;
    match error {
        sqlx::Error::Database(database_error) => {
            database_error
                .code()
                .as_deref()
                .map_or(PgErrorKind::Unknown, |code| match code {
                    str_constants::PG_SQLSTATE_STRING_DATA_RIGHT_TRUNCATION => {
                        PgErrorKind::StringDataRightTruncation
                    }
                    str_constants::PG_SQLSTATE_NUMERIC_VALUE_OUT_OF_RANGE => {
                        PgErrorKind::NumericValueOutOfRange
                    }
                    str_constants::PG_SQLSTATE_INVALID_TEXT_REPRESENTATION => {
                        PgErrorKind::InvalidTextRepresentation
                    }
                    str_constants::PG_SQLSTATE_NOT_NULL_VIOLATION => PgErrorKind::NotNullViolation,
                    str_constants::PG_SQLSTATE_FOREIGN_KEY_VIOLATION => {
                        PgErrorKind::ForeignKeyViolation
                    }
                    str_constants::PG_SQLSTATE_UNIQUE_VIOLATION => PgErrorKind::UniqueViolation,
                    str_constants::PG_SQLSTATE_CHECK_VIOLATION => PgErrorKind::CheckViolation,
                    str_constants::PG_SQLSTATE_SERIALIZATION_FAILURE => {
                        PgErrorKind::SerializationFailure
                    }
                    str_constants::PG_SQLSTATE_DEADLOCK_DETECTED => PgErrorKind::Deadlock,
                    _ => PgErrorKind::Unknown,
                })
        }
        sqlx::Error::PoolTimedOut => PgErrorKind::PoolTimedOut,
        sqlx::Error::Io(_) | sqlx::Error::Tls(_) | sqlx::Error::PoolClosed => {
            PgErrorKind::Connection
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
        | sqlx::Error::WorkerCrashed => PgErrorKind::Unknown,
        _ => std::convert::identity(PgErrorKind::Unknown),
    }
}

#[cfg(test)]
fn classify_pg_code(code: &str) -> PgErrorKind {
    match code {
        str_constants::PG_SQLSTATE_STRING_DATA_RIGHT_TRUNCATION => {
            PgErrorKind::StringDataRightTruncation
        }
        str_constants::PG_SQLSTATE_NUMERIC_VALUE_OUT_OF_RANGE => {
            PgErrorKind::NumericValueOutOfRange
        }
        str_constants::PG_SQLSTATE_INVALID_TEXT_REPRESENTATION => {
            PgErrorKind::InvalidTextRepresentation
        }
        str_constants::PG_SQLSTATE_NOT_NULL_VIOLATION => PgErrorKind::NotNullViolation,
        str_constants::PG_SQLSTATE_FOREIGN_KEY_VIOLATION => PgErrorKind::ForeignKeyViolation,
        str_constants::PG_SQLSTATE_UNIQUE_VIOLATION => PgErrorKind::UniqueViolation,
        str_constants::PG_SQLSTATE_CHECK_VIOLATION => PgErrorKind::CheckViolation,
        str_constants::PG_SQLSTATE_SERIALIZATION_FAILURE => PgErrorKind::SerializationFailure,
        str_constants::PG_SQLSTATE_DEADLOCK_DETECTED => PgErrorKind::Deadlock,
        _ => PgErrorKind::Unknown,
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn classifies_known_postgres_codes() {
        assert_eq!(
            super::classify_pg_code(str_constants::PG_SQLSTATE_UNIQUE_VIOLATION),
            super::PgErrorKind::UniqueViolation
        );
        assert_eq!(
            super::classify_pg_code(str_constants::PG_SQLSTATE_FOREIGN_KEY_VIOLATION),
            super::PgErrorKind::ForeignKeyViolation
        );
        assert_eq!(
            super::classify_pg_code(str_constants::PG_SQLSTATE_SERIALIZATION_FAILURE),
            super::PgErrorKind::SerializationFailure
        );
        assert_eq!(
            super::classify_pg_code(str_constants::PG_SQLSTATE_DEADLOCK_DETECTED),
            super::PgErrorKind::Deadlock
        );
    }

    #[test]
    fn classifies_unknown_postgres_code() {
        assert_eq!(
            super::classify_pg_code(str_constants::TEST_UNKNOWN_PG_SQLSTATE),
            super::PgErrorKind::Unknown
        );
    }

    #[test]
    fn classifies_non_database_errors() {
        assert_eq!(
            super::classify_pg_error(super::SqlxPgErrorRef::from(&sqlx::Error::PoolTimedOut)),
            super::PgErrorKind::PoolTimedOut
        );
        assert_eq!(
            super::classify_pg_error(super::SqlxPgErrorRef::from(&sqlx::Error::PoolClosed)),
            super::PgErrorKind::Connection
        );
        assert_eq!(
            super::classify_pg_error(super::SqlxPgErrorRef::from(&sqlx::Error::RowNotFound)),
            super::PgErrorKind::Unknown
        );
    }
}
