#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    proc_macro_newtype::FromInner,
)]
pub struct RetryAttemptsNonZeroUsize(std::num::NonZeroUsize);

impl RetryAttemptsNonZeroUsize {
    #[must_use]
    pub const fn get(self) -> usize {
        self.0.get()
    }
}

impl TryFrom<usize> for RetryAttemptsNonZeroUsize {
    type Error = crate::std_retry_attempts_error::StdRetryAttemptsError;

    fn try_from(usize: usize) -> Result<Self, Self::Error> {
        std::num::NonZeroUsize::new(usize)
            .map(Self)
            .ok_or(crate::std_retry_attempts_error::StdRetryAttemptsError::Zero)
    }
}
