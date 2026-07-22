const PG_RATE_LIMIT_KEY_PART_MAX_LEN: usize = 4096usize;

#[derive(Clone, Copy, Debug, newtype::FromInner)]
pub struct PgRateLimitQueryRef<'value_lt>(&'value_lt str);

#[derive(Clone, Copy, Debug, newtype::FromInner)]
pub struct SqlxPgRateLimitPoolRef<'value_lt>(&'value_lt sqlx::PgPool);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PgRateLimitMaximum(i64);
impl TryFrom<i64> for PgRateLimitMaximum {
    type Error = PgRateLimitValidationError;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        if value > 0i64 {
            Ok(Self(value))
        } else {
            Err(PgRateLimitValidationError::MustBePositive)
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PgRateLimitWindowSeconds(i32);
impl TryFrom<i32> for PgRateLimitWindowSeconds {
    type Error = PgRateLimitValidationError;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value > 0i32 {
            Ok(Self(value))
        } else {
            Err(PgRateLimitValidationError::MustBePositive)
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PgRateLimitDecision {
    Allowed,
    Limited(PgRateLimitWindowSeconds),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
pub enum PgRateLimitValidationError {
    #[error("rate-limit key part must not be empty")]
    EmptyKeyPart,
    #[error("rate-limit key part is too long")]
    KeyPartTooLong,
    #[error("rate-limit numeric configuration must be positive")]
    MustBePositive,
}

#[derive(
    Debug, newtype::ErrorTransparent, newtype::FromInner, newtype::IntoInnerFrom, newtype::Display,
)]
pub struct SqlxPgRateLimitError(sqlx::Error);

#[derive(Debug, thiserror::Error)]
pub enum PgRateLimitError {
    #[error("PostgreSQL rate-limit query failed: {0}")]
    Sqlx(SqlxPgRateLimitError),
}

pub async fn enforce_pg_rate_limit(
    pool: SqlxPgRateLimitPoolRef<'_>,
    query: PgRateLimitQueryRef<'_>,
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
            super::PgRateLimitMaximum::try_from(0i64),
            Err(super::PgRateLimitValidationError::MustBePositive)
        );
        assert_eq!(
            super::PgRateLimitScopeRef::try_from(str_constants::EMPTY),
            Err(super::PgRateLimitValidationError::EmptyKeyPart)
        );
    }
}
