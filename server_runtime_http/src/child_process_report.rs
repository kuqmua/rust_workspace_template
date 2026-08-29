#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug)]
// The owner module retains lint-sensitive semantics from the original implementation.
#[allow(clippy::arbitrary_source_item_ordering)]
pub struct ChildProcessReport {
    pub(super) diagnostic: crate::child_diagnostic::ChildDiagnostic,
    pub(super) status: crate::child_exit_status::ChildExitStatus,
    pub(super) completion: crate::child_process_completion::ChildProcessCompletion,
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
