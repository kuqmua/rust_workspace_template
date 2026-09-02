#[cfg(test)]
mod tests {
    #[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout)]
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
    fn empty_supervisor() -> crate::child_process_supervisor::ChildProcessSupervisor {
        crate::child_process_supervisor::ChildProcessSupervisor::default()
    }

    #[tokio::test]
    async fn test_process_set_enforces_capacity_and_identifier_overflow() {
        let mut full = crate::child_process_set::ChildProcessSet::new(
            crate::child_process_set_maximum_non_zero_usize::ChildProcessSetMaximumNonZeroUsize::from(std::num::NonZeroUsize::MIN),
        );
        assert_eq!(
            full.insert(empty_supervisor())
                .expect(constants_str::DIAGNOSTIC_806F6943),
            crate::child_process_id::ChildProcessId::from(constants_u64::ZERO)
        );
        assert!(matches!(
            full.insert(empty_supervisor()),
            Err(crate::child_process_set_error::ChildProcessSetError::Full)
        ));

        let mut overflowing = crate::child_process_set::ChildProcessSet::new(crate::child_process_set_maximum_non_zero_usize::ChildProcessSetMaximumNonZeroUsize::from(
            std::num::NonZeroUsize::new(2usize).expect(constants_str::DIAGNOSTIC_D96A312B),
        ));
        overflowing.set_next_id_for_test(crate::child_process_id::ChildProcessId::from(u64::MAX));
        assert!(matches!(
            overflowing.insert(empty_supervisor()),
            Err(crate::child_process_set_error::ChildProcessSetError::IdOverflow)
        ));
    }

    #[tokio::test]
    async fn test_missing_child_and_absent_diagnostic_are_explicit() {
        let timeout = crate::request_timeout_duration::RequestTimeoutDuration::try_from(
            std::time::Duration::from_secs(1u64),
        )
        .expect(constants_str::DIAGNOSTIC_02C5C4E9);
        assert!(matches!(
            empty_supervisor().shutdown(timeout).await,
            Err(crate::child_process_error::ChildProcessError::MissingChild)
        ));
        let diagnostic = crate::join_diagnostic::join_diagnostic(None)
            .await
            .expect(constants_str::DIAGNOSTIC_BFC19618);
        assert!(diagnostic.as_ref().is_empty());
    }

    #[tokio::test]
    async fn test_diagnostic_read_propagates_reader_errors() {
        let result = crate::read_child_diagnostic::read_child_diagnostic(
            ErrorReader,
            crate::child_diagnostic_maximum_non_zero_usize::ChildDiagnosticMaximumNonZeroUsize::from(std::num::NonZeroUsize::MIN),
        )
        .await;
        assert!(matches!(
            result,
            Err(crate::child_process_error::ChildProcessError::DiagnosticIo(
                _
            ))
        ));
    }

    #[tokio::test]
    async fn test_empty_process_set_shuts_down_without_reports() {
        let processes = crate::child_process_set::ChildProcessSet::new(
            crate::child_process_set_maximum_non_zero_usize::ChildProcessSetMaximumNonZeroUsize::from(std::num::NonZeroUsize::MIN),
        );
        let timeout = crate::request_timeout_duration::RequestTimeoutDuration::try_from(
            std::time::Duration::from_secs(1u64),
        )
        .expect(constants_str::DIAGNOSTIC_69D0D988);
        let reports = processes
            .shutdown_all(timeout)
            .await
            .expect(constants_str::DIAGNOSTIC_B85CBF78);
        assert!(reports.as_ref().is_empty());
    }

    #[tokio::test]
    async fn test_diagnostic_read_is_bounded() {
        let (mut writer, reader) = tokio::io::duplex(64usize);
        let write = tokio::spawn(async move {
            tokio::io::AsyncWriteExt::write_all(&mut writer, b"123456")
                .await
                .expect(constants_str::DIAGNOSTIC_248F268D);
        });
        let diagnostic = crate::read_child_diagnostic::read_child_diagnostic(
            reader,
            crate::child_diagnostic_maximum_non_zero_usize::ChildDiagnosticMaximumNonZeroUsize::from(
                std::num::NonZeroUsize::new(4usize)
                    .expect(constants_str::DIAGNOSTIC_9DE989AA),
            ),
        )
        .await
        .expect(constants_str::DIAGNOSTIC_35F4E073);
        write.await.expect(constants_str::DIAGNOSTIC_F859FB47);
        assert_eq!(diagnostic.as_ref(), b"1234");
    }
}

// Root-owned module compatibility wrappers.
