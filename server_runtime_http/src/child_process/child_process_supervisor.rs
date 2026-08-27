#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug)]
#[must_use]
pub struct ChildProcessSupervisor {
    pub(super) child: Option<super::TokioManagedChild>,
    pub(super) diagnostic: Option<super::TokioChildDiagnosticTask>,
}

impl ChildProcessSupervisor {
    pub fn new(
        mut child: super::TokioChildProcess,
        maximum: super::ChildDiagnosticMaximumNonZeroUsize,
    ) -> Self {
        let diagnostic = child.0.stderr.take().map(|stderr| {
            super::TokioChildDiagnosticTask::from(tokio::spawn(async move {
                super::read_child_diagnostic(stderr, maximum).await
            }))
        });
        Self {
            child: Some(super::TokioManagedChild::from(child.0)),
            diagnostic,
        }
    }

    pub async fn shutdown(
        mut self,
        timeout: crate::domain_types::RequestTimeoutDuration,
    ) -> Result<super::ChildProcessReport, super::ChildProcessError> {
        let mut child = self
            .child
            .take()
            .ok_or(super::ChildProcessError::MissingChild)?;
        let (completion, status) = match tokio::time::timeout(timeout.get(), child.0.wait()).await {
            Ok(result) => (
                super::ChildProcessCompletion::Exited,
                result
                    .map_err(super::ChildProcessIoError::from)
                    .map_err(super::ChildProcessError::Io)?,
            ),
            Err(_graceful_elapsed) => {
                child
                    .0
                    .start_kill()
                    .map_err(super::ChildProcessIoError::from)
                    .map_err(super::ChildProcessError::Io)?;
                let status = tokio::time::timeout(timeout.get(), child.0.wait())
                    .await
                    .map_err(|_kill_elapsed| super::ChildProcessError::Timeout)?
                    .map_err(super::ChildProcessIoError::from)
                    .map_err(super::ChildProcessError::Io)?;
                (super::ChildProcessCompletion::KilledAfterTimeout, status)
            }
        };
        let diagnostic = super::join_diagnostic(self.diagnostic.take()).await?;
        Ok(super::ChildProcessReport {
            completion,
            diagnostic,
            status: super::ChildExitStatus::from(status),
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
