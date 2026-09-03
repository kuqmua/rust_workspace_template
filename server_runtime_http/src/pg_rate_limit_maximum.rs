#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    proc_macro_newtype_get_inner::GetInner,
)]
#[accessor(pub(crate))]
pub struct PgRateLimitMaximum(std::num::NonZeroI64);

impl TryFrom<i64> for PgRateLimitMaximum {
    type Error = crate::pg_rate_limit_validation_error::PgRateLimitValidationError;

    fn try_from(i64: i64) -> Result<Self, Self::Error> {
        std::num::NonZeroI64::new(i64)
            .filter(|non_zero_value| non_zero_value.get() > constants_i64::ZERO)
            .map(Self)
            .ok_or(
                crate::pg_rate_limit_validation_error::PgRateLimitValidationError::MustBePositive,
            )
    }
}
