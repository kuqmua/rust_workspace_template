#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Debug)]
#[cfg_attr(test, derive(Default))]
#[must_use]
pub struct ChildProcessSupervisor {
    child: Option<crate::tokio_managed_child::TokioManagedChild>,
    diagnostic: Option<crate::tokio_child_diagnostic_task::TokioChildDiagnosticTask>,
}

impl ChildProcessSupervisor {
    pub fn new(
        child: crate::tokio_child_process::TokioChildProcess,
        maximum: crate::child_diagnostic_maximum_non_zero_usize::ChildDiagnosticMaximumNonZeroUsize,
    ) -> Self {
        let mut child_process = child.into_inner();
        let diagnostic = child_process.stderr.take().map(|stderr| {
            crate::tokio_child_diagnostic_task::TokioChildDiagnosticTask::from(tokio::spawn(
                async move {
                    crate::read_child_diagnostic::read_child_diagnostic(stderr, maximum).await
                },
            ))
        });
        Self {
            child: Some(crate::tokio_managed_child::TokioManagedChild::from(
                child_process,
            )),
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
        let (completion, status) = match tokio::time::timeout(timeout.get(), child.wait()).await {
            Ok(result) => (
                crate::child_process_completion::ChildProcessCompletion::Exited,
                result
                    .map_err(crate::child_process_io_error::ChildProcessIoError::from)
                    .map_err(crate::child_process_error::ChildProcessError::Io)?,
            ),
            Err(_graceful_elapsed) => {
                child
                    .start_kill()
                    .map_err(crate::child_process_io_error::ChildProcessIoError::from)
                    .map_err(crate::child_process_error::ChildProcessError::Io)?;
                let status = tokio::time::timeout(timeout.get(), child.wait())
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
        Ok(crate::child_process_report::ChildProcessReport::new(
            diagnostic,
            crate::child_exit_status::ChildExitStatus::from(status),
            completion,
        ))
    }
}

impl Drop for ChildProcessSupervisor {
    fn drop(&mut self) {
        if let Some(child) = self.child.as_mut() {
            let _kill_result = child.start_kill();
        }
        if let Some(diagnostic) = self.diagnostic.take() {
            diagnostic.abort();
        }
    }
}
