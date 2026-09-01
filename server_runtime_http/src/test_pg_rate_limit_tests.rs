#[cfg(test)]
mod tests {
    #[test]
    fn test_configuration_and_key_parts_are_bounded() {
        assert_eq!(
            crate::pg_rate_limit_maximum::PgRateLimitMaximum::try_from(constants_i64::ZERO),
            Err(crate::pg_rate_limit_validation_error::PgRateLimitValidationError::MustBePositive)
        );
        assert_eq!(
            crate::pg_rate_limit_scope_ref::PgRateLimitScopeRef::try_from(constants_str::EMPTY),
            Err(crate::pg_rate_limit_validation_error::PgRateLimitValidationError::EmptyKeyPart)
        );
    }
    #[test]
    fn test_numeric_configuration_requires_positive_values() {
        assert_eq!(
            crate::pg_rate_limit_maximum::PgRateLimitMaximum::try_from(-constants_i64::ONE),
            Err(crate::pg_rate_limit_validation_error::PgRateLimitValidationError::MustBePositive)
        );
        let _maximum =
            crate::pg_rate_limit_maximum::PgRateLimitMaximum::try_from(constants_i64::ONE)
                .expect(constants_str::DIAGNOSTIC_1C63C380);
        assert_eq!(
            crate::pg_rate_limit_window_seconds::PgRateLimitWindowSeconds::try_from(
                constants_i32::ZERO
            ),
            Err(crate::pg_rate_limit_validation_error::PgRateLimitValidationError::MustBePositive)
        );
        assert_eq!(
            crate::pg_rate_limit_window_seconds::PgRateLimitWindowSeconds::try_from(-1i32),
            Err(crate::pg_rate_limit_validation_error::PgRateLimitValidationError::MustBePositive)
        );
        let _window = crate::pg_rate_limit_window_seconds::PgRateLimitWindowSeconds::try_from(1i32)
            .expect(constants_str::DIAGNOSTIC_A5726134);
    }
    #[test]
    fn test_scope_and_subject_accept_exact_limit_and_reject_excess() {
        let exact = constants_str::A_ALT
            .repeat(crate::pg_rate_limit_key_part_max_len::PG_RATE_LIMIT_KEY_PART_MAX_LEN);
        let _scope = crate::pg_rate_limit_scope_ref::PgRateLimitScopeRef::try_from(exact.as_str())
            .expect(constants_str::DIAGNOSTIC_1B100A47);
        let _subject =
            crate::pg_rate_limit_subject_ref::PgRateLimitSubjectRef::try_from(exact.as_str())
                .expect(constants_str::DIAGNOSTIC_082E2933);
        let excess = constants_str::A_ALT.repeat(
            crate::pg_rate_limit_key_part_max_len::PG_RATE_LIMIT_KEY_PART_MAX_LEN
                + constants_usize::ONE,
        );
        assert_eq!(
            crate::pg_rate_limit_scope_ref::PgRateLimitScopeRef::try_from(excess.as_str()),
            Err(crate::pg_rate_limit_validation_error::PgRateLimitValidationError::KeyPartTooLong)
        );
        assert_eq!(
            crate::pg_rate_limit_subject_ref::PgRateLimitSubjectRef::try_from(excess.as_str()),
            Err(crate::pg_rate_limit_validation_error::PgRateLimitValidationError::KeyPartTooLong)
        );
        assert_eq!(
            crate::pg_rate_limit_subject_ref::PgRateLimitSubjectRef::try_from(constants_str::EMPTY),
            Err(crate::pg_rate_limit_validation_error::PgRateLimitValidationError::EmptyKeyPart)
        );
    }
}

// Root-owned module compatibility wrappers.
