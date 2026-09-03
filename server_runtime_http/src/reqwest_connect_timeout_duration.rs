#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    proc_macro_newtype_deref_inner::DerefInner,
)]
pub struct ReqwestConnectTimeoutDuration(std::time::Duration);
impl TryFrom<std::time::Duration> for ReqwestConnectTimeoutDuration {
    type Error = super::std_reqwest_timeout_error::StdReqwestTimeoutError;

    fn try_from(duration: std::time::Duration) -> Result<Self, Self::Error> {
        super::std_reqwest_timeout_duration_ref::StdReqwestTimeoutDurationRef::from(&duration)
            .validate()
            .map(|()| Self(duration))
    }
}
