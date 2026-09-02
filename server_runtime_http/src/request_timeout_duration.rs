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
pub struct RequestTimeoutDuration(std::time::Duration);

impl TryFrom<std::time::Duration> for RequestTimeoutDuration {
    type Error =
        crate::std_request_timeout_try_from_duration_error::StdRequestTimeoutTryFromDurationError;

    fn try_from(value: std::time::Duration) -> Result<Self, Self::Error> {
        if value.is_zero() {
            Err(crate::std_request_timeout_try_from_duration_error::StdRequestTimeoutTryFromDurationError::Zero)
        } else {
            Ok(Self(value))
        }
    }
}
