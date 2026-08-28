#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq)]
#[allow(clippy::arbitrary_source_item_ordering)] // alignment order required by optimal_memory_layout takes precedence over alphabetical field order
pub struct SynchronizationRuntimeConfiguration {
    retry_policy: server_runtime_core::RetryPolicy,
    execution_mode: server_runtime_core::ExecutionMode,
}

impl SynchronizationRuntimeConfiguration {
    #[must_use]
    pub const fn execution_mode(&self) -> server_runtime_core::ExecutionMode {
        self.execution_mode
    }

    #[must_use]
    pub const fn new(
        retry_policy: server_runtime_core::RetryPolicy,
        execution_mode: server_runtime_core::ExecutionMode,
    ) -> Self {
        Self {
            retry_policy,
            execution_mode,
        }
    }

    #[must_use]
    pub const fn retry_policy(&self) -> server_runtime_core::RetryPolicy {
        self.retry_policy
    }
}
