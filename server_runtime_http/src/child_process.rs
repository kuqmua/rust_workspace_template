#[path = "child_diagnostic.rs"]
mod child_diagnostic;
#[path = "child_diagnostic_maximum_non_zero_usize.rs"]
mod child_diagnostic_maximum_non_zero_usize;
#[path = "child_exit_status.rs"]
mod child_exit_status;
#[path = "child_process_completion.rs"]
mod child_process_completion;
#[path = "child_process_error.rs"]
mod child_process_error;
#[path = "child_process_id.rs"]
mod child_process_id;
#[path = "child_process_io_error.rs"]
mod child_process_io_error;
#[path = "child_process_report.rs"]
mod child_process_report;
#[path = "child_process_reports.rs"]
mod child_process_reports;
#[path = "child_process_set.rs"]
mod child_process_set;
#[path = "child_process_set_error.rs"]
mod child_process_set_error;
#[path = "child_process_set_maximum_non_zero_usize.rs"]
mod child_process_set_maximum_non_zero_usize;
#[path = "child_process_succeeded.rs"]
mod child_process_succeeded;
#[path = "child_process_supervisor.rs"]
mod child_process_supervisor;
#[path = "join_diagnostic.rs"]
mod join_diagnostic;
#[path = "read_child_diagnostic.rs"]
mod read_child_diagnostic;
#[path = "std_collections_child_process_map.rs"]
mod std_collections_child_process_map;
#[path = "tokio_child_diagnostic_task.rs"]
mod tokio_child_diagnostic_task;
#[path = "tokio_child_process.rs"]
mod tokio_child_process;
#[path = "tokio_child_process_join_error.rs"]
mod tokio_child_process_join_error;
#[path = "tokio_managed_child.rs"]
mod tokio_managed_child;

pub use child_diagnostic::ChildDiagnostic;
pub use child_diagnostic_maximum_non_zero_usize::ChildDiagnosticMaximumNonZeroUsize;
pub use child_exit_status::ChildExitStatus;
pub use child_process_completion::ChildProcessCompletion;
pub use child_process_error::ChildProcessError;
pub use child_process_id::ChildProcessId;
pub use child_process_io_error::ChildProcessIoError;
pub use child_process_report::ChildProcessReport;
pub use child_process_reports::ChildProcessReports;
pub use child_process_set::ChildProcessSet;
pub use child_process_set_error::ChildProcessSetError;
pub use child_process_set_maximum_non_zero_usize::ChildProcessSetMaximumNonZeroUsize;
pub use child_process_succeeded::ChildProcessSucceeded;
pub use child_process_supervisor::ChildProcessSupervisor;
use join_diagnostic::join_diagnostic;
use read_child_diagnostic::read_child_diagnostic;
use std_collections_child_process_map::StdCollectionsChildProcessMap;
use tokio_child_diagnostic_task::TokioChildDiagnosticTask;
pub use tokio_child_process::TokioChildProcess;
pub use tokio_child_process_join_error::TokioChildProcessJoinError;
use tokio_managed_child::TokioManagedChild;

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
