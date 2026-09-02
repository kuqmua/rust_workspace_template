#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    proc_macro_newtype::GetInner,
)]
#[accessor(pub(crate))]
pub struct PgRateLimitWindowSeconds(std::num::NonZeroI32);

impl TryFrom<i32> for PgRateLimitWindowSeconds {
    type Error = crate::pg_rate_limit_validation_error::PgRateLimitValidationError;

    fn try_from(i32: i32) -> Result<Self, Self::Error> {
        std::num::NonZeroI32::new(i32)
            .filter(|non_zero_value| non_zero_value.get() > constants_i32::ZERO)
            .map(Self)
            .ok_or(
                crate::pg_rate_limit_validation_error::PgRateLimitValidationError::MustBePositive,
            )
    }
}
