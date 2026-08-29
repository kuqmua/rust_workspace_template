pub use super::child_diagnostic::ChildDiagnostic;
pub use super::child_diagnostic_maximum_non_zero_usize::ChildDiagnosticMaximumNonZeroUsize;
pub use super::child_exit_status::ChildExitStatus;
pub use super::child_process_completion::ChildProcessCompletion;
pub use super::child_process_error::ChildProcessError;
pub use super::child_process_id::ChildProcessId;
pub use super::child_process_io_error::ChildProcessIoError;
pub use super::child_process_report::ChildProcessReport;
pub use super::child_process_reports::ChildProcessReports;
pub use super::child_process_set::ChildProcessSet;
pub use super::child_process_set_error::ChildProcessSetError;
pub use super::child_process_set_maximum_non_zero_usize::ChildProcessSetMaximumNonZeroUsize;
pub use super::child_process_succeeded::ChildProcessSucceeded;
pub use super::child_process_supervisor::ChildProcessSupervisor;
use super::join_diagnostic::join_diagnostic;
use super::read_child_diagnostic::read_child_diagnostic;
use super::std_collections_child_process_map::StdCollectionsChildProcessMap;
use super::tokio_child_diagnostic_task::TokioChildDiagnosticTask;
pub use super::tokio_child_process::TokioChildProcess;
pub use super::tokio_child_process_join_error::TokioChildProcessJoinError;
use super::tokio_managed_child::TokioManagedChild;
#[cfg(test)]
mod tests {
    #[derive(optimal_memory_layout::OptimalMemoryLayout)]
    struct ErrorReader;
    impl tokio::io::AsyncRead for ErrorReader {
        fn poll_read(
            self: std::pin::Pin<&mut Self>,
            _context: &mut std::task::Context<'_>,
            _buffer: &mut tokio::io::ReadBuf<'_>,
        ) -> std::task::Poll<std::io::Result<()>> {
            std::task::Poll::Ready(Err(std::io::Error::other(constants_str::VALUE_0DEDD057)))
        }
    }
    fn empty_supervisor() -> super::ChildProcessSupervisor {
        super::ChildProcessSupervisor {
            child: None,
            diagnostic: None,
        }
    }

    #[tokio::test]
    async fn process_set_enforces_capacity_and_identifier_overflow() {
        let mut full = super::ChildProcessSet::new(
            super::ChildProcessSetMaximumNonZeroUsize::from(std::num::NonZeroUsize::MIN),
        );
        assert_eq!(
            full.insert(empty_supervisor()).expect(
                "806f6943 process_set_enforces_capacity_and_identifier_overflow invariant must hold"
            ),
            super::ChildProcessId::from(constants_u64::ZERO)
        );
        assert!(matches!(
            full.insert(empty_supervisor()),
            Err(super::ChildProcessSetError::Full)
        ));

        let mut overflowing = super::ChildProcessSet::new(super::ChildProcessSetMaximumNonZeroUsize::from(
            std::num::NonZeroUsize::new(2usize).expect("d96a312b process_set_enforces_capacity_and_identifier_overflow invariant must hold"),
        ));
        overflowing.next_id = super::ChildProcessId::from(u64::MAX);
        assert!(matches!(
            overflowing.insert(empty_supervisor()),
            Err(super::ChildProcessSetError::IdOverflow)
        ));
    }

    #[tokio::test]
    async fn missing_child_and_absent_diagnostic_are_explicit() {
        let timeout = crate::domain_types::RequestTimeoutDuration::try_from(
            std::time::Duration::from_secs(1u64),
        )
        .expect("02c5c4e9 missing_child_and_absent_diagnostic_are_explicit invariant must hold");
        assert!(matches!(
            empty_supervisor().shutdown(timeout).await,
            Err(super::ChildProcessError::MissingChild)
        ));
        let diagnostic = super::join_diagnostic(None).await.expect(
            "bfc19618 missing_child_and_absent_diagnostic_are_explicit invariant must hold",
        );
        assert!(diagnostic.as_ref().is_empty());
    }

    #[tokio::test]
    async fn diagnostic_read_propagates_reader_errors() {
        let result = super::read_child_diagnostic(
            ErrorReader,
            super::ChildDiagnosticMaximumNonZeroUsize::from(std::num::NonZeroUsize::MIN),
        )
        .await;
        assert!(matches!(
            result,
            Err(super::ChildProcessError::DiagnosticIo(_))
        ));
    }

    #[tokio::test]
    async fn empty_process_set_shuts_down_without_reports() {
        let processes = super::ChildProcessSet::new(
            super::ChildProcessSetMaximumNonZeroUsize::from(std::num::NonZeroUsize::MIN),
        );
        let timeout = crate::domain_types::RequestTimeoutDuration::try_from(
            std::time::Duration::from_secs(1u64),
        )
        .expect("69d0d988 empty_process_set_shuts_down_without_reports invariant must hold");
        let reports = processes
            .shutdown_all(timeout)
            .await
            .expect("b85cbf78 empty_process_set_shuts_down_without_reports invariant must hold");
        assert!(reports.as_ref().is_empty());
    }

    #[tokio::test]
    async fn diagnostic_read_is_bounded() {
        let (mut writer, reader) = tokio::io::duplex(64usize);
        let write = tokio::spawn(async move {
            tokio::io::AsyncWriteExt::write_all(&mut writer, b"123456")
                .await
                .expect("248f268d diagnostic_read_is_bounded invariant must hold");
        });
        let diagnostic = super::read_child_diagnostic(
            reader,
            super::ChildDiagnosticMaximumNonZeroUsize::from(
                std::num::NonZeroUsize::new(4usize)
                    .expect("9de989aa diagnostic_read_is_bounded invariant must hold"),
            ),
        )
        .await
        .expect("35f4e073 diagnostic_read_is_bounded invariant must hold");
        write
            .await
            .expect("f859fb47 diagnostic_read_is_bounded invariant must hold");
        assert_eq!(diagnostic.as_ref(), b"1234");
    }
}

// Root-owned module compatibility wrappers.
mod child_diagnostic {
    pub use super::super::child_diagnostic::*;
}
mod child_diagnostic_maximum_non_zero_usize {
    pub use super::super::child_diagnostic_maximum_non_zero_usize::*;
}
mod child_exit_status {
    pub use super::super::child_exit_status::*;
}
mod child_process_completion {
    pub use super::super::child_process_completion::*;
}
mod child_process_error {
    pub use super::super::child_process_error::*;
}
mod child_process_id {
    pub use super::super::child_process_id::*;
}
mod child_process_io_error {
    pub use super::super::child_process_io_error::*;
}
mod child_process_report {
    pub use super::super::child_process_report::*;
}
mod child_process_reports {
    pub use super::super::child_process_reports::*;
}
mod child_process_set {
    pub use super::super::child_process_set::*;
}
mod child_process_set_error {
    pub use super::super::child_process_set_error::*;
}
mod child_process_set_maximum_non_zero_usize {
    pub use super::super::child_process_set_maximum_non_zero_usize::*;
}
mod child_process_succeeded {
    pub use super::super::child_process_succeeded::*;
}
mod child_process_supervisor {
    pub use super::super::child_process_supervisor::*;
}
mod join_diagnostic {
    pub use super::super::join_diagnostic::*;
}
mod read_child_diagnostic {
    pub use super::super::read_child_diagnostic::*;
}
mod std_collections_child_process_map {
    pub use super::super::std_collections_child_process_map::*;
}
mod tokio_child_diagnostic_task {
    pub use super::super::tokio_child_diagnostic_task::*;
}
mod tokio_child_process {
    pub use super::super::tokio_child_process::*;
}
mod tokio_child_process_join_error {
    pub use super::super::tokio_child_process_join_error::*;
}
mod tokio_managed_child {
    pub use super::super::tokio_managed_child::*;
}
