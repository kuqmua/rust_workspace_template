#[test]
fn test_database_error_classification_uses_sqlstate_and_handles_missing_codes() {
    #[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
    enum PgDatabaseClassificationFixtureError {
        #[error("{}", constants_str::X)]
        UniqueViolation,
        #[error("{}", constants_str::X)]
        Unknown,
        #[error("{}", constants_str::X)]
        Missing,
    }
    impl sqlx::error::DatabaseError for PgDatabaseClassificationFixtureError {
        fn message(&self) -> &str {
            constants_str::X
        }
        fn code(&self) -> Option<std::borrow::Cow<'_, str>> {
            match self {
                Self::UniqueViolation => Some(std::borrow::Cow::Borrowed(
                    constants_str::PG_SQLSTATE_UNIQUE_VIOLATION,
                )),
                Self::Unknown => Some(std::borrow::Cow::Owned(
                    constants_str::TEST_UNKNOWN_PG_SQLSTATE.to_owned(),
                )),
                Self::Missing => None,
            }
        }
        fn as_error(&self) -> &(dyn std::error::Error + Send + Sync + 'static) {
            self
        }
        fn as_error_mut(&mut self) -> &mut (dyn std::error::Error + Send + Sync + 'static) {
            self
        }
        fn into_error(self: Box<Self>) -> Box<dyn std::error::Error + Send + Sync + 'static> {
            self
        }
        fn kind(&self) -> sqlx::error::ErrorKind {
            sqlx::error::ErrorKind::Other
        }
    }
    assert!(
        [
            (
                PgDatabaseClassificationFixtureError::UniqueViolation,
                crate::pg_error_kind::PgErrorKind::UniqueViolation
            ),
            (
                PgDatabaseClassificationFixtureError::Unknown,
                crate::pg_error_kind::PgErrorKind::Unknown
            ),
            (
                PgDatabaseClassificationFixtureError::Missing,
                crate::pg_error_kind::PgErrorKind::Unknown
            ),
        ]
        .into_iter()
        .all(|(pg_database_classification_fixture_error, expected)| {
            let error = sqlx::Error::Database(Box::new(pg_database_classification_fixture_error));
            crate::classify_pg_error::classify_pg_error(
                crate::sqlx_pg_error_ref::SqlxPgErrorRef::from(&error),
            ) == expected
        })
    );
}

#[test]
fn test_sqlx_internal_error_variants_preserve_unknown_classification() {
    let source = || Box::new(std::io::Error::from(std::io::ErrorKind::InvalidData));
    assert!(
        [
            sqlx::Error::AnyDriverError(source()),
            sqlx::Error::BeginFailed,
            sqlx::Error::ColumnDecode {
                index: constants_str::X.to_owned(),
                source: source(),
            },
            sqlx::Error::ColumnIndexOutOfBounds {
                index: 1usize,
                len: 0usize
            },
            sqlx::Error::Configuration(source()),
            sqlx::Error::Decode(source()),
            sqlx::Error::Encode(source()),
            sqlx::Error::InvalidArgument(constants_str::X.to_owned()),
            sqlx::Error::InvalidSavePointStatement,
            sqlx::Error::Migrate(Box::new(sqlx::migrate::MigrateError::VersionMismatch(1i64))),
            sqlx::Error::TypeNotFound {
                type_name: constants_str::X.to_owned()
            },
        ]
        .into_iter()
        .all(|error| {
            crate::classify_pg_error::classify_pg_error(
                crate::sqlx_pg_error_ref::SqlxPgErrorRef::from(&error),
            ) == crate::pg_error_kind::PgErrorKind::Unknown
        })
    );
}
