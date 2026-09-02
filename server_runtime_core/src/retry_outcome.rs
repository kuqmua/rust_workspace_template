#[derive(proc_macro_getters::Getters)]
#[getters(bare)]
#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout, Debug, Eq, PartialEq, proc_macro_new::New,
)]
#[constructor(pub(crate))]
pub struct RetryOutcome<Success, Error> {
    #[getters(copy)]
    attempts: crate::retry_attempts_non_zero_usize::RetryAttemptsNonZeroUsize,
    result: Result<Success, Error>,
}

impl<Success, Error> RetryOutcome<Success, Error> {
    pub fn into_result(self) -> Result<Success, Error> {
        self.result
    }
}
