#[derive(generate_accessor::Getters)]
#[getters(bare)]
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    generate_constructor::New,
)]
pub struct RetryPolicy {
    #[getters(copy)]
    attempts: crate::retry_attempts_non_zero_usize::RetryAttemptsNonZeroUsize,
    #[getters(copy)]
    delay: Option<crate::retry_delay_duration::RetryDelayDuration>,
}
