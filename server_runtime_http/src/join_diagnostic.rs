// The owner module retains lint-sensitive semantics from the original implementation.

#[allow(
    clippy::single_call_fn,
    reason = "diagnostic task joining remains directly exercised by focused test_tests"
)]
pub(super) async fn join_diagnostic(
    option: Option<crate::tokio_child_diagnostic_task::TokioChildDiagnosticTask>,
) -> Result<crate::child_diagnostic::ChildDiagnostic, crate::child_process_error::ChildProcessError>
{
    match option {
        Some(diagnostic_task) => diagnostic_task
            .into_inner()
            .await
            .map_err(crate::tokio_child_process_join_error::TokioChildProcessJoinError::from)
            .map_err(crate::child_process_error::ChildProcessError::Join)?,
        None => Ok(crate::child_diagnostic::ChildDiagnostic::from(
            bounded_types::bounded_vec::BoundedVec::default(),
        )),
    }
}
