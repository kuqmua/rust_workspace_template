#[derive(proc_macro_getters::Getters)]
#[getters(bare)]
#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq,
)]
#[allow(
    clippy::arbitrary_source_item_ordering,
    reason = "lint suppression is required here"
)]
pub struct SynchronizationRuntimeConfiguration {
    #[getters(copy)]
    retry_policy: server_runtime_core::retry_policy::RetryPolicy,
    #[getters(copy)]
    execution_mode: server_runtime_core::execution_mode::ExecutionMode,
}

impl SynchronizationRuntimeConfiguration {
    #[must_use]
    pub const fn new(
        retry_policy: server_runtime_core::retry_policy::RetryPolicy,
        execution_mode: server_runtime_core::execution_mode::ExecutionMode,
    ) -> Self {
        Self {
            retry_policy,
            execution_mode,
        }
    }
}
