#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq)]
pub struct PgRateLimitMaximum(pub(super) std::num::NonZeroI64);

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
