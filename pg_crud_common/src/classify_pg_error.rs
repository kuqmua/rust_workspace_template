#[must_use]
pub fn classify_pg_error(
    error_ref: crate::domain_types::SqlxPgErrorRef<'_>,
) -> crate::domain_types::PgErrorKind {
    match error_ref.get() {
        sqlx::Error::Database(database_error) => database_error.code().as_deref().map_or(
            crate::domain_types::PgErrorKind::Unknown,
            crate::domain_types::classify_pg_code,
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
