#[derive(generate_accessor::Getters)]
#[getters(bare)]
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, generate_constructor::New)]
#[constructor(pub(crate))]
// The owner module retains lint-sensitive semantics from the original implementation.
#[allow(clippy::arbitrary_source_item_ordering)]
pub struct ChildProcessReport {
    diagnostic: crate::child_diagnostic::ChildDiagnostic,
    #[getters(copy)]
    status: crate::child_exit_status::ChildExitStatus,
    #[getters(copy)]
    completion: crate::child_process_completion::ChildProcessCompletion,
}
