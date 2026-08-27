#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq)]
pub struct PgRateLimitMaximum(
    pub(super) super::pg_rate_limit_maximum_non_zero_i64::PgRateLimitMaximumNonZeroI64,
);

impl TryFrom<i64> for PgRateLimitMaximum {
    type Error = super::PgRateLimitValidationError;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        std::num::NonZeroI64::new(value)
            .filter(|non_zero_value| non_zero_value.get() > constants_i64::ZERO)
            .map(super::pg_rate_limit_maximum_non_zero_i64::PgRateLimitMaximumNonZeroI64)
            .map(Self)
            .ok_or(super::PgRateLimitValidationError::MustBePositive)
    }
}
