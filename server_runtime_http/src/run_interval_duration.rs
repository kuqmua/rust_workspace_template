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
pub struct RunIntervalDuration(std::time::Duration);

impl TryFrom<std::time::Duration> for RunIntervalDuration {
    type Error =
        crate::std_run_interval_try_from_duration_error::StdRunIntervalTryFromDurationError;

    fn try_from(value: std::time::Duration) -> Result<Self, Self::Error> {
        (!value.is_zero()).then_some(Self(value)).ok_or(
            crate::std_run_interval_try_from_duration_error::StdRunIntervalTryFromDurationError::Zero,
        )
    }
}
