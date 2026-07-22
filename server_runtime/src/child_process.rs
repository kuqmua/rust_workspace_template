#[derive(Clone, Copy, Debug, Eq, PartialEq, newtype::FromInner)]
pub struct StdChildDiagnosticMaximum(std::num::NonZeroUsize);

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, newtype::FromInner)]
pub struct ChildProcessId(u64);

#[derive(Clone, Copy, Debug, Eq, PartialEq, newtype::FromInner)]
pub struct StdChildProcessSetMaximum(std::num::NonZeroUsize);

#[derive(Debug, Default, newtype::FromInner)]
struct StdCollectionsChildProcessMap(
    std::collections::BTreeMap<ChildProcessId, ChildProcessSupervisor>,
);

#[derive(Debug)]
pub struct ChildProcessSet {
    maximum: StdChildProcessSetMaximum,
    next_id: ChildProcessId,
    processes: StdCollectionsChildProcessMap,
}
impl ChildProcessSet {
    pub fn insert(
        &mut self,
        process: ChildProcessSupervisor,
    ) -> Result<ChildProcessId, ChildProcessSetError> {
        if self.processes.0.len() >= self.maximum.0.get() {
            return Err(ChildProcessSetError::Full);
        }
        let id = self.next_id;
        self.next_id = ChildProcessId::from(
            self.next_id
                .0
                .checked_add(1u64)
                .ok_or(ChildProcessSetError::IdOverflow)?,
        );
        let _previous = self.processes.0.insert(id, process);
        Ok(id)
    }

    #[must_use]
    pub fn new(maximum: StdChildProcessSetMaximum) -> Self {
        Self {
            maximum,
            next_id: ChildProcessId::from(0u64),
            processes: StdCollectionsChildProcessMap::from(std::collections::BTreeMap::new()),
        }
    }

    pub async fn shutdown_all(
        mut self,
        timeout: crate::StdRequestTimeout,
    ) -> Result<ChildProcessReports, ChildProcessSetError> {
        let mut reports = Vec::with_capacity(self.processes.0.len());
        while let Some((_id, process)) = self.processes.0.pop_first() {
            reports.push(
                process
                    .shutdown(timeout)
                    .await
                    .map_err(ChildProcessSetError::Process)?,
            );
        }
        Ok(ChildProcessReports::from(reports))
    }
}

#[derive(Clone, Debug, newtype::AsRefTarget, newtype::FromInner)]
pub struct ChildProcessReports(Vec<ChildProcessReport>);

#[derive(Debug, thiserror::Error)]
pub enum ChildProcessSetError {
    #[error("child process set is full")]
    Full,
    #[error("child process identifier overflowed")]
    IdOverflow,
    #[error("child process shutdown failed")]
    Process(#[source] ChildProcessError),
}

#[derive(Clone, Debug, Eq, PartialEq, newtype::AsRefTarget, newtype::FromInner)]
pub struct ChildDiagnostic(Vec<u8>);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ChildProcessCompletion {
    Exited,
    KilledAfterTimeout,
}

#[derive(Clone, Copy, Debug, newtype::FromInner)]
pub struct StdChildExitStatus(std::process::ExitStatus);

impl StdChildExitStatus {
    #[must_use]
    pub fn succeeded(self) -> ChildProcessSucceeded {
        if self.0.success() {
            ChildProcessSucceeded::Yes
        } else {
            ChildProcessSucceeded::No
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ChildProcessSucceeded {
    No,
    Yes,
}

#[derive(Clone, Debug)]
pub struct ChildProcessReport {
    completion: ChildProcessCompletion,
    diagnostic: ChildDiagnostic,
    status: StdChildExitStatus,
}
impl ChildProcessReport {
    #[must_use]
    pub const fn completion(&self) -> ChildProcessCompletion {
        self.completion
    }

    #[must_use]
    pub const fn diagnostic(&self) -> &ChildDiagnostic {
        &self.diagnostic
    }

    #[must_use]
    pub const fn status(&self) -> StdChildExitStatus {
        self.status
    }
}

#[derive(Debug, newtype::FromInner)]
struct TokioManagedChild(tokio::process::Child);

#[derive(Debug, newtype::FromInner)]
pub struct TokioChildProcess(tokio::process::Child);

#[derive(Debug, newtype::FromInner)]
struct TokioChildDiagnosticTask(
    tokio::task::JoinHandle<Result<ChildDiagnostic, ChildProcessError>>,
);

#[derive(Debug)]
#[must_use]
pub struct ChildProcessSupervisor {
    child: Option<TokioManagedChild>,
    diagnostic: Option<TokioChildDiagnosticTask>,
}
impl ChildProcessSupervisor {
    pub fn new(mut child: TokioChildProcess, maximum: StdChildDiagnosticMaximum) -> Self {
        let diagnostic = child.0.stderr.take().map(|stderr| {
            TokioChildDiagnosticTask::from(tokio::spawn(async move {
                read_child_diagnostic(stderr, maximum).await
            }))
        });
        Self {
            child: Some(TokioManagedChild::from(child.0)),
            diagnostic,
        }
    }

    pub async fn shutdown(
        mut self,
        timeout: crate::StdRequestTimeout,
    ) -> Result<ChildProcessReport, ChildProcessError> {
        let mut child = self.child.take().ok_or(ChildProcessError::MissingChild)?;
        let (completion, status) = match tokio::time::timeout(timeout.get(), child.0.wait()).await {
            Ok(result) => (
                ChildProcessCompletion::Exited,
                result
                    .map_err(StdChildProcessIoError::from)
                    .map_err(ChildProcessError::Io)?,
            ),
            Err(_graceful_elapsed) => {
                child
                    .0
                    .start_kill()
                    .map_err(StdChildProcessIoError::from)
                    .map_err(ChildProcessError::Io)?;
                let status = tokio::time::timeout(timeout.get(), child.0.wait())
                    .await
                    .map_err(|_kill_elapsed| ChildProcessError::Timeout)?
                    .map_err(StdChildProcessIoError::from)
                    .map_err(ChildProcessError::Io)?;
                (ChildProcessCompletion::KilledAfterTimeout, status)
            }
        };
        let diagnostic = join_diagnostic(self.diagnostic.take()).await?;
        Ok(ChildProcessReport {
            completion,
            diagnostic,
            status: StdChildExitStatus::from(status),
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

#[derive(Debug, thiserror::Error)]
pub enum ChildProcessError {
    #[error("child process diagnostic read failed")]
    DiagnosticIo(StdChildProcessIoError),
    #[error("child process diagnostic buffer range is invalid")]
    DiagnosticRange,
    #[error("child process operation failed")]
    Io(StdChildProcessIoError),
    #[error("child process diagnostic task failed")]
    Join(TokioChildProcessJoinError),
    #[error("child process is missing")]
    MissingChild,
    #[error("child process did not terminate before the timeout")]
    Timeout,
}

#[derive(Debug, thiserror::Error)]
#[error(transparent)]
#[derive(newtype::FromInner)]
pub struct StdChildProcessIoError(std::io::Error);

#[derive(Debug, thiserror::Error)]
#[error(transparent)]
#[derive(newtype::FromInner)]
pub struct TokioChildProcessJoinError(tokio::task::JoinError);

#[allow(clippy::single_call_fn)] // isolates optional diagnostic task joining from process state transitions
async fn join_diagnostic(
    optional_task: Option<TokioChildDiagnosticTask>,
) -> Result<ChildDiagnostic, ChildProcessError> {
    match optional_task {
        Some(diagnostic_task) => diagnostic_task
            .0
            .await
            .map_err(TokioChildProcessJoinError::from)
            .map_err(ChildProcessError::Join)?,
        None => Ok(ChildDiagnostic::from(Vec::new())),
    }
}

#[allow(clippy::single_call_fn)] // generic reader keeps bounded diagnostic behavior independently testable
async fn read_child_diagnostic<Reader>(
    mut reader: Reader,
    maximum: StdChildDiagnosticMaximum,
) -> Result<ChildDiagnostic, ChildProcessError>
where
    Reader: tokio::io::AsyncRead + Unpin,
{
    let mut output = Vec::with_capacity(maximum.0.get());
    let mut buffer = [0u8; 4096usize];
    while output.len() < maximum.0.get() {
        let remaining = maximum.0.get().saturating_sub(output.len());
        let read_length = remaining.min(buffer.len());
        let target = buffer
            .get_mut(..read_length)
            .ok_or(ChildProcessError::DiagnosticRange)?;
        let read = tokio::io::AsyncReadExt::read(&mut reader, target)
            .await
            .map_err(StdChildProcessIoError::from)
            .map_err(ChildProcessError::DiagnosticIo)?;
        if read == 0usize {
            break;
        }
        let read_bytes = buffer
            .get(..read)
            .ok_or(ChildProcessError::DiagnosticRange)?;
        output.extend_from_slice(read_bytes);
    }
    Ok(ChildDiagnostic::from(output))
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn empty_process_set_shuts_down_without_reports() {
        let processes = super::ChildProcessSet::new(super::StdChildProcessSetMaximum::from(
            std::num::NonZeroUsize::MIN,
        ));
        let timeout = crate::StdRequestTimeout::try_from(std::time::Duration::from_secs(1u64))
            .expect("69d0d988");
        let reports = processes.shutdown_all(timeout).await.expect("b85cbf78");
        assert!(reports.as_ref().is_empty());
    }

    #[tokio::test]
    async fn diagnostic_read_is_bounded() {
        let (mut writer, reader) = tokio::io::duplex(64usize);
        let write = tokio::spawn(async move {
            tokio::io::AsyncWriteExt::write_all(&mut writer, b"123456")
                .await
                .expect("248f268d");
        });
        let diagnostic = super::read_child_diagnostic(
            reader,
            super::StdChildDiagnosticMaximum::from(
                std::num::NonZeroUsize::new(4usize).expect("9de989aa"),
            ),
        )
        .await
        .expect("35f4e073");
        write.await.expect("f859fb47");
        assert_eq!(diagnostic.as_ref(), b"1234");
    }
}
