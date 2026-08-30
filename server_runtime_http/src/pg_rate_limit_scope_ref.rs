#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq)]
pub struct PgRateLimitScopeRef<'value_lt>(&'value_lt str);

impl<'value_lt> PgRateLimitScopeRef<'value_lt> {
    #[must_use]
    pub(crate) const fn get(self) -> &'value_lt str {
        self.0
    }
}

impl<'value_lt> TryFrom<&'value_lt str> for PgRateLimitScopeRef<'value_lt> {
    type Error = crate::pg_rate_limit_validation_error::PgRateLimitValidationError;

    fn try_from(value: &'value_lt str) -> Result<Self, Self::Error> {
        if value.is_empty() {
            Err(crate::pg_rate_limit_validation_error::PgRateLimitValidationError::EmptyKeyPart)
        } else if value.len()
            > crate::pg_rate_limit_key_part_max_len::PG_RATE_LIMIT_KEY_PART_MAX_LEN
        {
            Err(crate::pg_rate_limit_validation_error::PgRateLimitValidationError::KeyPartTooLong)
        } else {
            Ok(Self(value))
        }
    }
}
