#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    proc_macro_newtype::DerefInner,
)]
pub struct ReqwestRequestTimeoutDuration(std::time::Duration);
impl TryFrom<std::time::Duration> for ReqwestRequestTimeoutDuration {
    type Error = super::std_reqwest_timeout_error::StdReqwestTimeoutError;

    fn try_from(value: std::time::Duration) -> Result<Self, Self::Error> {
        super::std_reqwest_timeout_duration_ref::StdReqwestTimeoutDurationRef::from(&value)
            .validate()
            .map(|()| Self(value))
    }
}
