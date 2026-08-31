#[cfg(test)]
mod tests {
    #[test]
    fn configuration_and_key_parts_are_bounded() {
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
    fn numeric_configuration_requires_positive_values() {
        assert_eq!(
            crate::pg_rate_limit_maximum::PgRateLimitMaximum::try_from(-constants_i64::ONE),
            Err(crate::pg_rate_limit_validation_error::PgRateLimitValidationError::MustBePositive)
        );
        let _maximum = crate::pg_rate_limit_maximum::PgRateLimitMaximum::try_from(
            constants_i64::ONE,
        )
        .expect("1c63c380 numeric_configuration_requires_positive_values invariant must hold");
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
            .expect("a5726134 numeric_configuration_requires_positive_values invariant must hold");
    }
    #[test]
    fn scope_and_subject_accept_exact_limit_and_reject_excess() {
        let exact = constants_str::A_ALT
            .repeat(crate::pg_rate_limit_key_part_max_len::PG_RATE_LIMIT_KEY_PART_MAX_LEN);
        let _scope = crate::pg_rate_limit_scope_ref::PgRateLimitScopeRef::try_from(exact.as_str()).expect(
            "1b100a47 scope_and_subject_accept_exact_limit_and_reject_excess invariant must hold",
        );
        let _subject = crate::pg_rate_limit_subject_ref::PgRateLimitSubjectRef::try_from(
            exact.as_str(),
        )
        .expect(
            "082e2933 scope_and_subject_accept_exact_limit_and_reject_excess invariant must hold",
        );
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
mod enforce_pg_rate_limit {}
mod pg_rate_limit_decision {}
mod pg_rate_limit_error {}
mod pg_rate_limit_key_part_max_len {}
mod pg_rate_limit_maximum {}
mod pg_rate_limit_query_ref {}
mod pg_rate_limit_scope_ref {}
mod pg_rate_limit_subject_ref {}
mod pg_rate_limit_validation_error {}
mod pg_rate_limit_window_seconds {}
mod sqlx_pg_rate_limit_error {}
mod sqlx_pg_rate_limit_pool_ref {}
