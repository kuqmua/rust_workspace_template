#[derive(proc_macro_getters::Getters)]
#[getters(bare)]
#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    proc_macro_new::New,
)]
pub struct RetryPolicy {
    #[getters(copy)]
    attempts: crate::retry_attempts_non_zero_usize::RetryAttemptsNonZeroUsize,
    #[getters(copy)]
    delay: Option<crate::retry_delay_duration::RetryDelayDuration>,
}
