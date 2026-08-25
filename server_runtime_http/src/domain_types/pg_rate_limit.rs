const PG_RATE_LIMIT_KEY_PART_MAX_LEN: usize = 4096usize;

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, newtype::FromInner)]
pub struct PgRateLimitQueryRef(&'static str);

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, newtype::FromInner)]
pub struct SqlxPgRateLimitPoolRef<'value_lt>(&'value_lt sqlx::PgPool);

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq)]
pub struct PgRateLimitScopeRef<'value_lt>(&'value_lt str);
impl<'value_lt> TryFrom<&'value_lt str> for PgRateLimitScopeRef<'value_lt> {
    type Error = PgRateLimitValidationError;

    fn try_from(value: &'value_lt str) -> Result<Self, Self::Error> {
        if value.is_empty() {
            Err(PgRateLimitValidationError::EmptyKeyPart)
        } else if value.len() > PG_RATE_LIMIT_KEY_PART_MAX_LEN {
            Err(PgRateLimitValidationError::KeyPartTooLong)
        } else {
            Ok(Self(value))
        }
    }
}

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq)]
pub struct PgRateLimitSubjectRef<'value_lt>(&'value_lt str);
impl<'value_lt> TryFrom<&'value_lt str> for PgRateLimitSubjectRef<'value_lt> {
    type Error = PgRateLimitValidationError;

    fn try_from(value: &'value_lt str) -> Result<Self, Self::Error> {
        if value.is_empty() {
            Err(PgRateLimitValidationError::EmptyKeyPart)
        } else if value.len() > PG_RATE_LIMIT_KEY_PART_MAX_LEN {
            Err(PgRateLimitValidationError::KeyPartTooLong)
        } else {
            Ok(Self(value))
        }
    }
}

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq)]
pub struct PgRateLimitMaximum(i64);
impl TryFrom<i64> for PgRateLimitMaximum {
    type Error = PgRateLimitValidationError;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        if value > constants_i64::ZERO {
            Ok(Self(value))
        } else {
            Err(PgRateLimitValidationError::MustBePositive)
        }
    }
}

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq)]
pub struct PgRateLimitWindowSeconds(i32);
impl TryFrom<i32> for PgRateLimitWindowSeconds {
    type Error = PgRateLimitValidationError;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value > constants_i32::ZERO {
            Ok(Self(value))
        } else {
            Err(PgRateLimitValidationError::MustBePositive)
        }
    }
}

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq)]
pub enum PgRateLimitDecision {
    Allowed,
    Limited(PgRateLimitWindowSeconds),
}

#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error,
)]
pub enum PgRateLimitValidationError {
    #[error("rate-limit key part must not be empty")]
    EmptyKeyPart,
    #[error("rate-limit key part is too long")]
    KeyPartTooLong,
    #[error("rate-limit numeric configuration must be positive")]
    MustBePositive,
}

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    thiserror::Error,
    newtype::FromInner,
    newtype::IntoInnerFrom,
)]
#[error(transparent)]
pub struct SqlxPgRateLimitError(sqlx::Error);

#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
pub enum PgRateLimitError {
    #[error("PostgreSQL rate-limit query failed: {0}")]
    Sqlx(SqlxPgRateLimitError),
}

pub async fn enforce_pg_rate_limit(
    pool: SqlxPgRateLimitPoolRef<'_>,
    query: PgRateLimitQueryRef,
    scope: PgRateLimitScopeRef<'_>,
    subject: PgRateLimitSubjectRef<'_>,
    maximum: PgRateLimitMaximum,
    window_seconds: PgRateLimitWindowSeconds,
) -> Result<PgRateLimitDecision, PgRateLimitError> {
    sqlx::query_scalar::<_, bool>(query.0)
        .bind(scope.0)
        .bind(subject.0)
        .bind(maximum.0)
        .bind(window_seconds.0)
        .fetch_one(pool.0)
        .await
        .map(|allowed| {
            if allowed {
                PgRateLimitDecision::Allowed
            } else {
                PgRateLimitDecision::Limited(window_seconds)
            }
        })
        .map_err(|error| PgRateLimitError::Sqlx(SqlxPgRateLimitError::from(error)))
}

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
        let exact = "a".repeat(super::PG_RATE_LIMIT_KEY_PART_MAX_LEN);
        let _scope = super::PgRateLimitScopeRef::try_from(exact.as_str()).expect(
            "1b100a47 scope_and_subject_accept_exact_limit_and_reject_excess invariant must hold",
        );
        let _subject = super::PgRateLimitSubjectRef::try_from(exact.as_str()).expect(
            "082e2933 scope_and_subject_accept_exact_limit_and_reject_excess invariant must hold",
        );
        let excess = "a".repeat(super::PG_RATE_LIMIT_KEY_PART_MAX_LEN + constants_usize::ONE);
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
