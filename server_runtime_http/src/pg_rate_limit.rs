#[path = "enforce_pg_rate_limit.rs"]
mod enforce_pg_rate_limit;
#[path = "pg_rate_limit_decision.rs"]
mod pg_rate_limit_decision;
#[path = "pg_rate_limit_error.rs"]
mod pg_rate_limit_error;
#[path = "pg_rate_limit_key_part_max_len.rs"]
mod pg_rate_limit_key_part_max_len;
#[path = "pg_rate_limit_maximum.rs"]
mod pg_rate_limit_maximum;
#[path = "pg_rate_limit_maximum_non_zero_i64.rs"]
mod pg_rate_limit_maximum_non_zero_i64;
#[path = "pg_rate_limit_query_ref.rs"]
mod pg_rate_limit_query_ref;
#[path = "pg_rate_limit_scope_ref.rs"]
mod pg_rate_limit_scope_ref;
#[path = "pg_rate_limit_subject_ref.rs"]
mod pg_rate_limit_subject_ref;
#[path = "pg_rate_limit_validation_error.rs"]
mod pg_rate_limit_validation_error;
#[path = "pg_rate_limit_window_seconds.rs"]
mod pg_rate_limit_window_seconds;
#[path = "pg_rate_limit_window_seconds_non_zero_i32.rs"]
mod pg_rate_limit_window_seconds_non_zero_i32;
#[path = "sqlx_pg_rate_limit_error.rs"]
mod sqlx_pg_rate_limit_error;
#[path = "sqlx_pg_rate_limit_pool_ref.rs"]
mod sqlx_pg_rate_limit_pool_ref;

pub use enforce_pg_rate_limit::enforce_pg_rate_limit;
pub use pg_rate_limit_decision::PgRateLimitDecision;
pub use pg_rate_limit_error::PgRateLimitError;
use pg_rate_limit_key_part_max_len::PG_RATE_LIMIT_KEY_PART_MAX_LEN;
pub use pg_rate_limit_maximum::PgRateLimitMaximum;
pub use pg_rate_limit_query_ref::PgRateLimitQueryRef;
pub use pg_rate_limit_scope_ref::PgRateLimitScopeRef;
pub use pg_rate_limit_subject_ref::PgRateLimitSubjectRef;
pub use pg_rate_limit_validation_error::PgRateLimitValidationError;
pub use pg_rate_limit_window_seconds::PgRateLimitWindowSeconds;
pub use sqlx_pg_rate_limit_error::SqlxPgRateLimitError;
pub use sqlx_pg_rate_limit_pool_ref::SqlxPgRateLimitPoolRef;

#[cfg(test)]
mod tests {
    #[test]
    fn configuration_and_key_parts_are_bounded() {
        assert_eq!(
            super::PgRateLimitMaximum::try_from(constants_i64::ZERO),
            Err(super::PgRateLimitValidationError::MustBePositive)
        );
        assert_eq!(
            super::PgRateLimitScopeRef::try_from(constants_str::EMPTY),
            Err(super::PgRateLimitValidationError::EmptyKeyPart)
        );
    }
    #[test]
    fn numeric_configuration_requires_positive_values() {
        assert_eq!(
            super::PgRateLimitMaximum::try_from(-constants_i64::ONE),
            Err(super::PgRateLimitValidationError::MustBePositive)
        );
        let _maximum = super::PgRateLimitMaximum::try_from(constants_i64::ONE)
            .expect("1c63c380 numeric_configuration_requires_positive_values invariant must hold");
        assert_eq!(
            super::PgRateLimitWindowSeconds::try_from(constants_i32::ZERO),
            Err(super::PgRateLimitValidationError::MustBePositive)
        );
        assert_eq!(
            super::PgRateLimitWindowSeconds::try_from(-1i32),
            Err(super::PgRateLimitValidationError::MustBePositive)
        );
        let _window = super::PgRateLimitWindowSeconds::try_from(1i32)
            .expect("a5726134 numeric_configuration_requires_positive_values invariant must hold");
    }
    #[test]
    fn scope_and_subject_accept_exact_limit_and_reject_excess() {
        let exact = constants_str::A_ALT.repeat(super::PG_RATE_LIMIT_KEY_PART_MAX_LEN);
        let _scope = super::PgRateLimitScopeRef::try_from(exact.as_str()).expect(
            "1b100a47 scope_and_subject_accept_exact_limit_and_reject_excess invariant must hold",
        );
        let _subject = super::PgRateLimitSubjectRef::try_from(exact.as_str()).expect(
            "082e2933 scope_and_subject_accept_exact_limit_and_reject_excess invariant must hold",
        );
        let excess = constants_str::A_ALT
            .repeat(super::PG_RATE_LIMIT_KEY_PART_MAX_LEN + constants_usize::ONE);
        assert_eq!(
            super::PgRateLimitScopeRef::try_from(excess.as_str()),
            Err(super::PgRateLimitValidationError::KeyPartTooLong)
        );
        assert_eq!(
            super::PgRateLimitSubjectRef::try_from(excess.as_str()),
            Err(super::PgRateLimitValidationError::KeyPartTooLong)
        );
        assert_eq!(
            super::PgRateLimitSubjectRef::try_from(constants_str::EMPTY),
            Err(super::PgRateLimitValidationError::EmptyKeyPart)
        );
    }
}
