#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug)]
#[must_use]
pub struct ChildProcessSupervisor {
    pub(super) child: Option<crate::tokio_managed_child::TokioManagedChild>,
    pub(super) diagnostic: Option<crate::tokio_child_diagnostic_task::TokioChildDiagnosticTask>,
}

impl ChildProcessSupervisor {
    pub fn new(
        mut child: crate::tokio_child_process::TokioChildProcess,
        maximum: crate::child_diagnostic_maximum_non_zero_usize::ChildDiagnosticMaximumNonZeroUsize,
    ) -> Self {
        let diagnostic = child.0.stderr.take().map(|stderr| {
            crate::tokio_child_diagnostic_task::TokioChildDiagnosticTask::from(tokio::spawn(
                async move {
                    crate::read_child_diagnostic::read_child_diagnostic(stderr, maximum).await
                },
            ))
        });
        Self {
            child: Some(crate::tokio_managed_child::TokioManagedChild::from(child.0)),
            diagnostic,
        }
    }

    pub async fn shutdown(
        mut self,
        timeout: crate::request_timeout_duration::RequestTimeoutDuration,
    ) -> Result<
        crate::child_process_report::ChildProcessReport,
        crate::child_process_error::ChildProcessError,
    > {
        let mut child = self
            .child
            .take()
            .ok_or(crate::child_process_error::ChildProcessError::MissingChild)?;
        let (completion, status) = match tokio::time::timeout(timeout.get(), child.0.wait()).await {
            Ok(result) => (
                crate::child_process_completion::ChildProcessCompletion::Exited,
                result
                    .map_err(crate::child_process_io_error::ChildProcessIoError::from)
                    .map_err(crate::child_process_error::ChildProcessError::Io)?,
            ),
            Err(_graceful_elapsed) => {
                child
                    .0
                    .start_kill()
                    .map_err(crate::child_process_io_error::ChildProcessIoError::from)
                    .map_err(crate::child_process_error::ChildProcessError::Io)?;
                let status = tokio::time::timeout(timeout.get(), child.0.wait())
                    .await
                    .map_err(|_kill_elapsed| {
                        crate::child_process_error::ChildProcessError::Timeout
                    })?
                    .map_err(crate::child_process_io_error::ChildProcessIoError::from)
                    .map_err(crate::child_process_error::ChildProcessError::Io)?;
                (
                    crate::child_process_completion::ChildProcessCompletion::KilledAfterTimeout,
                    status,
                )
            }
        };
        let diagnostic = crate::join_diagnostic::join_diagnostic(self.diagnostic.take()).await?;
        Ok(crate::child_process_report::ChildProcessReport {
            completion,
            diagnostic,
            status: crate::child_exit_status::ChildExitStatus::from(status),
        })
    }
}

impl Drop for ChildProcessSupervisor {
    fn drop(&mut self) {
        if let Some(child) = self.child.as_mut() {
            let _kill_result = child.0.start_kill();
        }
        if let Some(diagnostic) = self.diagnostic.take() {
            diagnostic.0.abort();
        }
    }
}
