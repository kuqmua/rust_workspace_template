#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    proc_macro_newtype::FromInner,
)]
pub(super) struct StdReqwestTimeoutDurationRef<'value_lt>(&'value_lt std::time::Duration);
impl StdReqwestTimeoutDurationRef<'_> {
    pub(super) const fn validate(
        self,
    ) -> Result<(), super::std_reqwest_timeout_error::StdReqwestTimeoutError> {
        if self.0.is_zero() {
            Err(super::std_reqwest_timeout_error::StdReqwestTimeoutError::Zero)
        } else {
            Ok(())
        }
    }
}
