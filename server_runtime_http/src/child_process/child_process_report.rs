#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug)]
#[allow(clippy::arbitrary_source_item_ordering)]
pub struct ChildProcessReport {
    pub(super) diagnostic: super::ChildDiagnostic,
    pub(super) status: super::ChildExitStatus,
    pub(super) completion: super::ChildProcessCompletion,
}

impl ChildProcessReport {
    #[must_use]
    pub const fn completion(&self) -> super::ChildProcessCompletion {
        self.completion
    }

    #[must_use]
    pub const fn diagnostic(&self) -> &super::ChildDiagnostic {
        &self.diagnostic
    }

    #[must_use]
    pub const fn status(&self) -> super::ChildExitStatus {
        self.status
    }
}
