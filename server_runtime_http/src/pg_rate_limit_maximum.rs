#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq)]
pub struct PgRateLimitMaximum(std::num::NonZeroI64);

impl PgRateLimitMaximum {
    pub(crate) const fn get(self) -> std::num::NonZeroI64 {
        self.0
    }
}

impl TryFrom<i64> for PgRateLimitMaximum {
    type Error = crate::pg_rate_limit_validation_error::PgRateLimitValidationError;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        std::num::NonZeroI64::new(value)
            .filter(|non_zero_value| non_zero_value.get() > constants_i64::ZERO)
            .map(Self)
            .ok_or(
                crate::pg_rate_limit_validation_error::PgRateLimitValidationError::MustBePositive,
            )
    }
}
