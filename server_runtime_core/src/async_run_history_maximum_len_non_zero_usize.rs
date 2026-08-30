#[allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the history owner module reads the validated private maximum length"
)]
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    newtype::DerefInner,
)]
pub struct AsyncRunHistoryMaximumLenNonZeroUsize(std::num::NonZeroUsize);
impl TryFrom<usize> for AsyncRunHistoryMaximumLenNonZeroUsize {
    type Error = super::std_async_run_history_maximum_len_try_from_usize_error::StdAsyncRunHistoryMaximumLenTryFromUsizeError;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        std::num::NonZeroUsize::new(value)
            .map(Self)
            .ok_or(super::std_async_run_history_maximum_len_try_from_usize_error::StdAsyncRunHistoryMaximumLenTryFromUsizeError::Zero)
    }
}
