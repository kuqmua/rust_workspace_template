#[must_use]
#[allow(
    clippy::single_call_fn,
    reason = "classify pg code remains a named owner because its boundary role is clearer and directly testable"
)]
pub(crate) fn classify_pg_code(str: &str) -> crate::pg_error_kind::PgErrorKind {
    match str {
        constants_str::PG_SQLSTATE_STRING_DATA_RIGHT_TRUNCATION => {
            crate::pg_error_kind::PgErrorKind::StringDataRightTruncation
        }
        constants_str::PG_SQLSTATE_NUMERIC_VALUE_OUT_OF_RANGE => {
            crate::pg_error_kind::PgErrorKind::NumericValueOutOfRange
        }
        constants_str::PG_SQLSTATE_INVALID_TEXT_REPRESENTATION => {
            crate::pg_error_kind::PgErrorKind::InvalidTextRepresentation
        }
        constants_str::PG_SQLSTATE_NOT_NULL_VIOLATION => {
            crate::pg_error_kind::PgErrorKind::NotNullViolation
        }
        constants_str::PG_SQLSTATE_FOREIGN_KEY_VIOLATION => {
            crate::pg_error_kind::PgErrorKind::ForeignKeyViolation
        }
        constants_str::PG_SQLSTATE_UNIQUE_VIOLATION => {
            crate::pg_error_kind::PgErrorKind::UniqueViolation
        }
        constants_str::PG_SQLSTATE_CHECK_VIOLATION => {
            crate::pg_error_kind::PgErrorKind::CheckViolation
        }
        constants_str::PG_SQLSTATE_SERIALIZATION_FAILURE => {
            crate::pg_error_kind::PgErrorKind::SerializationFailure
        }
        constants_str::PG_SQLSTATE_DEADLOCK_DETECTED => crate::pg_error_kind::PgErrorKind::Deadlock,
        _ => crate::pg_error_kind::PgErrorKind::Unknown,
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_classifies_known_postgres_codes() {
        assert_eq!(
            crate::classify_pg_code::classify_pg_code(constants_str::PG_SQLSTATE_UNIQUE_VIOLATION),
            crate::pg_error_kind::PgErrorKind::UniqueViolation
        );
        assert_eq!(
            crate::classify_pg_code::classify_pg_code(
                constants_str::PG_SQLSTATE_FOREIGN_KEY_VIOLATION
            ),
            crate::pg_error_kind::PgErrorKind::ForeignKeyViolation
        );
        assert_eq!(
            crate::classify_pg_code::classify_pg_code(
                constants_str::PG_SQLSTATE_SERIALIZATION_FAILURE
            ),
            crate::pg_error_kind::PgErrorKind::SerializationFailure
        );
        assert_eq!(
            crate::classify_pg_code::classify_pg_code(constants_str::PG_SQLSTATE_DEADLOCK_DETECTED),
            crate::pg_error_kind::PgErrorKind::Deadlock
        );
    }

    #[test]
    fn test_classifies_unknown_postgres_code() {
        assert_eq!(
            crate::classify_pg_code::classify_pg_code(constants_str::TEST_UNKNOWN_PG_SQLSTATE),
            crate::pg_error_kind::PgErrorKind::Unknown
        );
    }
}
