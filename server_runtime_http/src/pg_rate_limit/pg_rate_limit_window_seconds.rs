#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq)]
pub struct PgRateLimitWindowSeconds(
    pub(super) super::pg_rate_limit_window_seconds_non_zero_i32::PgRateLimitWindowSecondsNonZeroI32,
);

impl TryFrom<i32> for PgRateLimitWindowSeconds {
    type Error = super::PgRateLimitValidationError;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        std::num::NonZeroI32::new(value)
            .filter(|non_zero_value| non_zero_value.get() > constants_i32::ZERO)
            .map(
                super::pg_rate_limit_window_seconds_non_zero_i32::PgRateLimitWindowSecondsNonZeroI32,
            )
            .map(Self)
            .ok_or(super::PgRateLimitValidationError::MustBePositive)
    }
}
