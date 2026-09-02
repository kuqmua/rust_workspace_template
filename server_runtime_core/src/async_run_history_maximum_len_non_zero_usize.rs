#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    proc_macro_newtype::DerefInner,
)]
pub struct AsyncRunHistoryMaximumLenNonZeroUsize(std::num::NonZeroUsize);
impl TryFrom<usize> for AsyncRunHistoryMaximumLenNonZeroUsize {
    type Error = super::std_async_run_history_maximum_len_try_from_usize_error::StdAsyncRunHistoryMaximumLenTryFromUsizeError;
    fn try_from(usize: usize) -> Result<Self, Self::Error> {
        std::num::NonZeroUsize::new(usize)
            .map(Self)
            .ok_or(super::std_async_run_history_maximum_len_try_from_usize_error::StdAsyncRunHistoryMaximumLenTryFromUsizeError::Zero)
    }
}
