#[derive(proc_macro_getters::Getters)]
#[getters(bare)]
#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq,
)]
#[allow(
    clippy::arbitrary_source_item_ordering,
    reason = "synchronization runtime configuration keeps declaration order aligned with generated layout or processing flow"
)]
#[derive(proc_macro_new::New)]
pub struct SynchronizationRuntimeConfiguration {
    #[getters(copy)]
    retry_policy: server_runtime_core::retry_policy::RetryPolicy,
    #[getters(copy)]
    execution_mode: server_runtime_core::execution_mode::ExecutionMode,
}
