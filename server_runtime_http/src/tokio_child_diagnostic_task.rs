#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, newtype::FromInner)]
pub(super) struct TokioChildDiagnosticTask(
    pub(super)  tokio::task::JoinHandle<
        Result<
            crate::child_diagnostic::ChildDiagnostic,
            crate::child_process_error::ChildProcessError,
        >,
    >,
);
