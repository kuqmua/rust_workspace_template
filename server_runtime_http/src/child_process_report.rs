#[derive(proc_macro_getters::Getters)]
#[getters(bare)]
#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, proc_macro_new::New,
)]
#[constructor(pub(crate))]
#[allow(
    clippy::arbitrary_source_item_ordering,
    reason = "lint suppression is required here"
)]
pub struct ChildProcessReport {
    diagnostic: crate::child_diagnostic::ChildDiagnostic,
    #[getters(copy)]
    status: crate::child_exit_status::ChildExitStatus,
    #[getters(copy)]
    completion: crate::child_process_completion::ChildProcessCompletion,
}
