#[allow(
    clippy::single_call_fn,
    reason = "diagnostic task joining remains directly exercised by focused child process tests"
)]
pub(super) async fn join_diagnostic(
    option: Option<&mut crate::tokio_child_diagnostic_task::TokioChildDiagnosticTask>,
    request_timeout_duration: crate::request_timeout_duration::RequestTimeoutDuration,
) -> Result<crate::child_diagnostic::ChildDiagnostic, crate::child_process_error::ChildProcessError>
{
    match option {
        Some(diagnostic_task) => {
            tokio::time::timeout(request_timeout_duration.get(), diagnostic_task.join())
                .await
                .map_err(crate::tokio_child_diagnostic_elapsed::TokioChildDiagnosticElapsed::from)
                .map_err(crate::child_process_error::ChildProcessError::DiagnosticTimeout)?
        }
        None => Ok(crate::child_diagnostic::ChildDiagnostic::from(
            bounded_types::bounded_vec::BoundedVec::default(),
        )),
    }
}
