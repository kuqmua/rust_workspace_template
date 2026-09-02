#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq,
)]
pub struct PgRateLimitWindowSeconds(std::num::NonZeroI32);

impl PgRateLimitWindowSeconds {
    pub(crate) const fn get(self) -> std::num::NonZeroI32 {
        self.0
    }
}

impl TryFrom<i32> for PgRateLimitWindowSeconds {
    type Error = crate::pg_rate_limit_validation_error::PgRateLimitValidationError;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        std::num::NonZeroI32::new(value)
            .filter(|non_zero_value| non_zero_value.get() > constants_i32::ZERO)
            .map(Self)
            .ok_or(
                crate::pg_rate_limit_validation_error::PgRateLimitValidationError::MustBePositive,
            )
    }
}
