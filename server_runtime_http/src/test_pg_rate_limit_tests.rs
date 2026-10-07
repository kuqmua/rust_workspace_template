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
    fn test_rate_limit_numeric_endpoints_preserve_positive_values_and_reject_nonpositive_values() {
        assert!([i64::MIN, -1i64, 0i64, 1i64, i64::MAX].into_iter().all(|value| {
            let result = crate::pg_rate_limit_maximum::PgRateLimitMaximum::try_from(value);
            if value > 0i64 {
                result.is_ok_and(|maximum| maximum.get().get() == value)
            } else {
                result == Err(crate::pg_rate_limit_validation_error::PgRateLimitValidationError::MustBePositive)
            }
        }));
        assert!([i32::MIN, -1i32, 0i32, 1i32, i32::MAX].into_iter().all(|value| {
            let result = crate::pg_rate_limit_window_seconds::PgRateLimitWindowSeconds::try_from(value);
            if value > 0i32 {
                result.is_ok_and(|window| window.get().get() == value)
            } else {
                result == Err(crate::pg_rate_limit_validation_error::PgRateLimitValidationError::MustBePositive)
            }
        }));
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
    #[test]
    fn test_rate_limit_key_parts_measure_utf8_bytes_and_preserve_borrowed_text() {
        let exact = '\u{e9}'.to_string().repeat(2048usize);
        let below = format!(
            "{}{}",
            '\u{e9}'.to_string().repeat(2047usize),
            constants_str::X
        );
        let above = format!("{exact}{}", constants_str::X);
        assert!([
            (below.as_str(), true), (exact.as_str(), true), (above.as_str(), false),
        ].into_iter().all(|(text, valid)| {
            let scope = crate::pg_rate_limit_scope_ref::PgRateLimitScopeRef::try_from(text);
            let subject = crate::pg_rate_limit_subject_ref::PgRateLimitSubjectRef::try_from(text);
            if valid {
                scope.is_ok_and(|value| value.get() == text && value.get().as_ptr() == text.as_ptr())
                    && subject.is_ok_and(|value| value.get() == text && value.get().as_ptr() == text.as_ptr())
            } else {
                scope == Err(crate::pg_rate_limit_validation_error::PgRateLimitValidationError::KeyPartTooLong)
                    && subject == Err(crate::pg_rate_limit_validation_error::PgRateLimitValidationError::KeyPartTooLong)
            }
        }));
    }
}
