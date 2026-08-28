// The owner module retains lint-sensitive semantics from the original implementation.

#[allow(
    clippy::single_call_fn,
    reason = "diagnostic task joining remains directly exercised by focused tests"
)]
pub(super) async fn join_diagnostic(
    optional_task: Option<super::TokioChildDiagnosticTask>,
) -> Result<super::ChildDiagnostic, super::ChildProcessError> {
    match optional_task {
        Some(diagnostic_task) => diagnostic_task
            .0
            .await
            .map_err(super::TokioChildProcessJoinError::from)
            .map_err(super::ChildProcessError::Join)?,
        None => Ok(super::ChildDiagnostic::from(
            bounded_types::domain_types::vector::BoundedVec::default(),
        )),
    }
}
