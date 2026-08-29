#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq)]
pub struct RunIntervalDuration(pub(super) std::time::Duration);

impl TryFrom<std::time::Duration> for RunIntervalDuration {
    type Error =
        crate::std_run_interval_try_from_duration_error::StdRunIntervalTryFromDurationError;

    fn try_from(value: std::time::Duration) -> Result<Self, Self::Error> {
        if value.is_zero() {
            Err(crate::std_run_interval_try_from_duration_error::StdRunIntervalTryFromDurationError)
        } else {
            Ok(Self(value))
        }
    }
}
