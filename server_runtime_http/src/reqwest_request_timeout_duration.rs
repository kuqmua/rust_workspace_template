#[allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the HTTP client owner reads this validated timeout across owner modules"
)]
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, newtype::DerefInner)]
pub struct ReqwestRequestTimeoutDuration(std::time::Duration);
impl TryFrom<std::time::Duration> for ReqwestRequestTimeoutDuration {
    type Error = super::std_reqwest_timeout_error::StdReqwestTimeoutError;

    fn try_from(value: std::time::Duration) -> Result<Self, Self::Error> {
        super::std_reqwest_timeout_duration_ref::StdReqwestTimeoutDurationRef::from(&value)
            .validate()
            .map(|()| Self(value))
    }
}
