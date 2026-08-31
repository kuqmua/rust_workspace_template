#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, generate_constructor::New)]
#[constructor(pub(crate))]
// The owner module retains lint-sensitive semantics from the original implementation.
#[allow(clippy::arbitrary_source_item_ordering)]
pub struct ChildProcessReport {
    diagnostic: crate::child_diagnostic::ChildDiagnostic,
    status: crate::child_exit_status::ChildExitStatus,
    completion: crate::child_process_completion::ChildProcessCompletion,
}

impl ChildProcessReport {
    #[must_use]
    pub const fn completion(&self) -> crate::child_process_completion::ChildProcessCompletion {
        self.completion
    }

    #[must_use]
    pub const fn diagnostic(&self) -> &crate::child_diagnostic::ChildDiagnostic {
        &self.diagnostic
    }

    #[must_use]
    pub const fn status(&self) -> crate::child_exit_status::ChildExitStatus {
        self.status
    }
}
