#![allow(
    unused_variables,
    reason = "test I/O trait fixtures preserve repository type-based parameter names"
)]

#[cfg(test)]
mod tests {
    #[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout)]
    struct ErrorReader;
    impl tokio::io::AsyncRead for ErrorReader {
        fn poll_read(
            self: std::pin::Pin<&mut Self>,
            context: &mut std::task::Context<'_>,
            read_buf: &mut tokio::io::ReadBuf<'_>,
        ) -> std::task::Poll<std::io::Result<()>> {
            std::task::Poll::Ready(Err(std::io::Error::other(constants_str::VALUE_0DEDD057)))
        }
    }
    fn diagnostic_join_timeout_fixture() -> crate::request_timeout_duration::RequestTimeoutDuration
    {
        crate::request_timeout_duration::RequestTimeoutDuration::try_from(
            std::time::Duration::from_secs(1u64),
        )
        .expect(constants_str::DIAGNOSTIC_02C5C4E9)
    }

    fn empty_supervisor() -> crate::child_process_supervisor::ChildProcessSupervisor {
        crate::child_process_supervisor::ChildProcessSupervisor::default()
    }

    #[tokio::test(start_paused = true)]
    async fn test_diagnostic_join_preserves_completed_payload_and_inner_error() {
        let diagnostic = crate::child_diagnostic::ChildDiagnostic::from(
            bounded_types::bounded_vec::BoundedVec::from_max_iter(
                constants_str::X.as_bytes().iter().copied(),
            ),
        );
        let mut successful_task =
            crate::tokio_child_diagnostic_task::TokioChildDiagnosticTask::from(tokio::spawn(
                std::future::ready(Ok(diagnostic.clone())),
            ));
        let successful_join = crate::join_diagnostic::join_diagnostic(
            Some(&mut successful_task),
            diagnostic_join_timeout_fixture(),
        )
        .await;
        assert!(successful_join.is_ok_and(|payload| payload == diagnostic));
        let mut failed_task = crate::tokio_child_diagnostic_task::TokioChildDiagnosticTask::from(
            tokio::spawn(std::future::ready(Err(
                crate::child_process_error::ChildProcessError::DiagnosticRange,
            ))),
        );
        assert!(matches!(
            crate::join_diagnostic::join_diagnostic(
                Some(&mut failed_task),
                diagnostic_join_timeout_fixture(),
            )
            .await,
            Err(crate::child_process_error::ChildProcessError::DiagnosticRange)
        ));
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
        let timeout = diagnostic_join_timeout_fixture();
        assert!(matches!(
            empty_supervisor().shutdown(timeout).await,
            Err(crate::child_process_error::ChildProcessError::MissingChild)
        ));
        let diagnostic = crate::join_diagnostic::join_diagnostic(None, timeout)
            .await
            .expect(constants_str::DIAGNOSTIC_BFC19618);
        assert!(diagnostic.as_ref().is_empty());
    }

    #[tokio::test(start_paused = true)]
    async fn test_cancelled_diagnostic_join_preserves_task_ownership() {
        let mut diagnostic_task =
            crate::tokio_child_diagnostic_task::TokioChildDiagnosticTask::from(tokio::spawn(
                std::future::pending(),
            ));
        async {
            let mut diagnostic_join = std::pin::pin!(crate::join_diagnostic::join_diagnostic(
                Some(&mut diagnostic_task),
                diagnostic_join_timeout_fixture(),
            ));
            std::future::poll_fn(|context| {
                assert!(diagnostic_join.as_mut().poll(context).is_pending());
                std::task::Poll::Ready(())
            })
            .await;
        }
        .await;
        diagnostic_task.abort();
        assert!(matches!(
            crate::join_diagnostic::join_diagnostic(
                Some(&mut diagnostic_task),
                diagnostic_join_timeout_fixture(),
            )
            .await,
            Err(crate::child_process_error::ChildProcessError::Join(_))
        ));
    }

    #[tokio::test(start_paused = true)]
    async fn test_diagnostic_join_has_a_deadline_and_retains_ownership() {
        let mut diagnostic_task =
            crate::tokio_child_diagnostic_task::TokioChildDiagnosticTask::from(tokio::spawn(
                std::future::pending(),
            ));
        let result = tokio::time::timeout(
            std::time::Duration::from_secs(2u64),
            crate::join_diagnostic::join_diagnostic(
                Some(&mut diagnostic_task),
                diagnostic_join_timeout_fixture(),
            ),
        )
        .await;
        diagnostic_task.abort();
        assert!(matches!(
            crate::join_diagnostic::join_diagnostic(
                Some(&mut diagnostic_task),
                diagnostic_join_timeout_fixture(),
            )
            .await,
            Err(crate::child_process_error::ChildProcessError::Join(_))
        ));
        assert!(matches!(
            result,
            Ok(Err(error))
                if matches!(&error, crate::child_process_error::ChildProcessError::DiagnosticTimeout(_))
                    && std::error::Error::source(&error)
                        .is_some_and(<dyn std::error::Error>::is::<tokio::time::error::Elapsed>)
        ));
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
    async fn test_diagnostic_read_does_not_allocate_maximum_upfront() {
        let maximum = crate::child_diagnostic_maximum_non_zero_usize::ChildDiagnosticMaximumNonZeroUsize::from(
            std::num::NonZeroUsize::MAX,
        );
        assert!(matches!(
            crate::read_child_diagnostic::read_child_diagnostic(tokio::io::empty(), maximum).await,
            Ok(diagnostic) if diagnostic.as_ref().is_empty()
        ));
        let reader = tokio::io::AsyncReadExt::take(tokio::io::repeat(1u8), 4097u64);
        assert!(matches!(
            crate::read_child_diagnostic::read_child_diagnostic(reader, maximum).await,
            Ok(diagnostic) if diagnostic.as_ref().len() == 4097usize
                && diagnostic.as_ref().iter().all(|byte| *byte == 1u8)
        ));
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

    #[tokio::test]
    async fn test_diagnostic_capture_limit_keeps_the_writer_open_until_eof() {
        let (mut writer, reader) = tokio::io::duplex(4usize);
        let write = tokio::spawn(async move {
            tokio::io::AsyncWriteExt::write_all(
                &mut writer,
                constants_str::X.repeat(16usize).as_bytes(),
            )
            .await
        });
        let diagnostic = crate::read_child_diagnostic::read_child_diagnostic(
            reader,
            crate::child_diagnostic_maximum_non_zero_usize::ChildDiagnosticMaximumNonZeroUsize::from(
                std::num::NonZeroUsize::MIN,
            ),
        )
        .await;
        let write_result = write.await;
        assert!(matches!(
            diagnostic,
            Ok(value) if value.as_ref() == constants_str::X.as_bytes()
        ));
        assert!(matches!(write_result, Ok(Ok(()))));
    }
    #[tokio::test]
    async fn test_nonempty_process_set_preserves_shutdown_error() {
        let mut processes = crate::child_process_set::ChildProcessSet::new(
            crate::child_process_set_maximum_non_zero_usize::ChildProcessSetMaximumNonZeroUsize::from(std::num::NonZeroUsize::MIN),
        );
        assert!(matches!(
            processes.insert(empty_supervisor()),
            Ok(crate::child_process_id::ChildProcessId { .. })
        ));
        assert!(matches!(
            processes
                .shutdown_all(diagnostic_join_timeout_fixture())
                .await,
            Err(
                crate::child_process_set_error::ChildProcessSetError::Process(
                    crate::child_process_error::ChildProcessError::MissingChild
                )
            )
        ));
    }
}
