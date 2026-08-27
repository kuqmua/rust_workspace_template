#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "split owner modules expose representation only within the crate"
)]
#[must_use]
#[allow(
    clippy::single_call_fn,
    reason = "SQLSTATE classification remains isolated in its named owner module"
)]
pub(crate) fn classify_pg_code(code: &str) -> crate::domain_types::PgErrorKind {
    match code {
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
        constants_str::PG_SQLSTATE_DEADLOCK_DETECTED => crate::domain_types::PgErrorKind::Deadlock,
        _ => crate::domain_types::PgErrorKind::Unknown,
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn classifies_known_postgres_codes() {
        assert_eq!(
            super::classify_pg_code(constants_str::PG_SQLSTATE_UNIQUE_VIOLATION),
            crate::domain_types::PgErrorKind::UniqueViolation
        );
        assert_eq!(
            super::classify_pg_code(constants_str::PG_SQLSTATE_FOREIGN_KEY_VIOLATION),
            crate::domain_types::PgErrorKind::ForeignKeyViolation
        );
        assert_eq!(
            super::classify_pg_code(constants_str::PG_SQLSTATE_SERIALIZATION_FAILURE),
            crate::domain_types::PgErrorKind::SerializationFailure
        );
        assert_eq!(
            super::classify_pg_code(constants_str::PG_SQLSTATE_DEADLOCK_DETECTED),
            crate::domain_types::PgErrorKind::Deadlock
        );
    }

    #[test]
    fn classifies_unknown_postgres_code() {
        assert_eq!(
            super::classify_pg_code(constants_str::TEST_UNKNOWN_PG_SQLSTATE),
            crate::domain_types::PgErrorKind::Unknown
        );
    }
}
