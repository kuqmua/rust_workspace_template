#[must_use]
pub fn classify_pg_error(
    error_ref: crate::sqlx_pg_error_ref::SqlxPgErrorRef<'_>,
) -> crate::pg_error_kind::PgErrorKind {
    match error_ref.get() {
        sqlx::Error::Database(database_error) => database_error.code().as_deref().map_or(
            crate::pg_error_kind::PgErrorKind::Unknown,
            crate::classify_pg_code::classify_pg_code,
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
