#![allow(
    unused_crate_dependencies,
    reason = "integration target exercises child process supervision from the HTTP runtime crate"
)]

#[cfg(test)]
mod tests {
    #[tokio::test]
    #[ignore = "requires true and false executables on PATH; run in a provisioned process environment"]
    async fn test_child_process_completion_preserves_exit_status_and_empty_diagnostic() {
        let check_completion = async |child_process_succeeded: server_runtime_http::child_process_succeeded::ChildProcessSucceeded| {
            let (program, stderr) = match child_process_succeeded {
                server_runtime_http::child_process_succeeded::ChildProcessSucceeded::Yes => (
                    constants_str::TRUE,
                    std::process::Stdio::piped(),
                ),
                server_runtime_http::child_process_succeeded::ChildProcessSucceeded::No => (
                    constants_str::FALSE,
                    std::process::Stdio::null(),
                ),
            };
            let child_result = tokio::process::Command::new(program).stderr(stderr).spawn();
            assert!(child_result.is_ok());
            let Ok(child) = child_result else {
                return;
            };
            let supervisor = server_runtime_http::child_process_supervisor::ChildProcessSupervisor::new(
                server_runtime_http::tokio_child_process::TokioChildProcess::from(child),
                server_runtime_http::child_diagnostic_maximum_non_zero_usize::ChildDiagnosticMaximumNonZeroUsize::from(std::num::NonZeroUsize::MIN),
            );
            let timeout_result = server_runtime_http::request_timeout_duration::RequestTimeoutDuration::try_from(std::time::Duration::from_secs(60u64));
            assert_ne!(timeout_result, Err(server_runtime_http::std_request_timeout_try_from_duration_error::StdRequestTimeoutTryFromDurationError::Zero));
            let Ok(timeout) = timeout_result else {
                return;
            };
            let report_result = supervisor.shutdown(timeout).await;
            assert!(report_result.is_ok());
            let Ok(report) = report_result else {
                return;
            };
            assert_eq!(report.completion(), server_runtime_http::child_process_completion::ChildProcessCompletion::Exited);
            assert_eq!(report.status().succeeded(), child_process_succeeded);
            assert_eq!(report.diagnostic(), &server_runtime_http::child_diagnostic::ChildDiagnostic::from(bounded_types::bounded_vec::BoundedVec::default()));
        };
        check_completion(server_runtime_http::child_process_succeeded::ChildProcessSucceeded::Yes)
            .await;
        check_completion(server_runtime_http::child_process_succeeded::ChildProcessSucceeded::No)
            .await;
    }
    #[tokio::test]
    #[ignore = "requires printf executable on PATH; run in a provisioned process environment"]
    async fn test_child_process_diagnostic_is_truncated_without_changing_failed_exit() {
        let child_result = tokio::process::Command::new(constants_str::PRINTF)
            .stderr(std::process::Stdio::piped())
            .spawn();
        assert!(child_result.is_ok());
        let Ok(child) = child_result else {
            return;
        };
        let supervisor = server_runtime_http::child_process_supervisor::ChildProcessSupervisor::new(
            server_runtime_http::tokio_child_process::TokioChildProcess::from(child),
            server_runtime_http::child_diagnostic_maximum_non_zero_usize::ChildDiagnosticMaximumNonZeroUsize::from(std::num::NonZeroUsize::MIN),
        );
        let timeout_result =
            server_runtime_http::request_timeout_duration::RequestTimeoutDuration::try_from(
                std::time::Duration::from_secs(60u64),
            );
        assert_ne!(timeout_result, Err(server_runtime_http::std_request_timeout_try_from_duration_error::StdRequestTimeoutTryFromDurationError::Zero));
        let Ok(timeout) = timeout_result else {
            return;
        };
        let report_result = supervisor.shutdown(timeout).await;
        assert!(report_result.is_ok());
        let Ok(report) = report_result else {
            return;
        };
        assert_eq!(
            report.completion(),
            server_runtime_http::child_process_completion::ChildProcessCompletion::Exited
        );
        assert_eq!(
            report.status().succeeded(),
            server_runtime_http::child_process_succeeded::ChildProcessSucceeded::No
        );
        assert_eq!(report.diagnostic().as_ref().len(), 1usize);
    }
    #[tokio::test(start_paused = true)]
    #[ignore = "requires head executable on PATH; run in a provisioned process environment"]
    async fn test_child_process_shutdown_kills_pending_child_after_deadline() {
        let child_result = tokio::process::Command::new(constants_str::HEAD)
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .spawn();
        assert!(child_result.is_ok());
        let Ok(mut child) = child_result else {
            return;
        };
        let stdin = child.stdin.take();
        assert!(stdin.is_some());
        let supervisor = server_runtime_http::child_process_supervisor::ChildProcessSupervisor::new(
            server_runtime_http::tokio_child_process::TokioChildProcess::from(child),
            server_runtime_http::child_diagnostic_maximum_non_zero_usize::ChildDiagnosticMaximumNonZeroUsize::from(std::num::NonZeroUsize::MIN),
        );
        let duration = std::time::Duration::from_secs(60u64);
        let timeout_result =
            server_runtime_http::request_timeout_duration::RequestTimeoutDuration::try_from(
                duration,
            );
        assert_ne!(timeout_result, Err(server_runtime_http::std_request_timeout_try_from_duration_error::StdRequestTimeoutTryFromDurationError::Zero));
        let Ok(timeout) = timeout_result else {
            return;
        };
        let mut shutdown = std::pin::pin!(supervisor.shutdown(timeout));
        let pending = std::future::poll_fn(|context| {
            std::task::Poll::Ready(shutdown.as_mut().poll(context).is_pending())
        })
        .await;
        assert!(pending);
        tokio::time::advance(duration).await;
        tokio::time::resume();
        let report_result = shutdown.await;
        drop(stdin);
        assert!(report_result.is_ok());
        let Ok(report) = report_result else {
            return;
        };
        assert_eq!(report.completion(), server_runtime_http::child_process_completion::ChildProcessCompletion::KilledAfterTimeout);
        assert_eq!(
            report.status().succeeded(),
            server_runtime_http::child_process_succeeded::ChildProcessSucceeded::No
        );
        assert_eq!(
            report.diagnostic(),
            &server_runtime_http::child_diagnostic::ChildDiagnostic::from(
                bounded_types::bounded_vec::BoundedVec::default()
            )
        );
    }
    #[tokio::test]
    #[ignore = "requires head executable on PATH; run in a provisioned process environment"]
    async fn test_dropping_child_supervisor_closes_pending_child_output() {
        let check_drop = async |request_timeout_duration: Option<
            server_runtime_http::request_timeout_duration::RequestTimeoutDuration,
        >| {
            let child_result = tokio::process::Command::new(constants_str::HEAD)
                .stdin(std::process::Stdio::piped())
                .stdout(std::process::Stdio::piped())
                .stderr(std::process::Stdio::piped())
                .spawn();
            assert!(child_result.is_ok());
            let Ok(mut child) = child_result else {
                return;
            };
            let stdin = child.stdin.take();
            let stdout_option = child.stdout.take();
            let supervisor = server_runtime_http::child_process_supervisor::ChildProcessSupervisor::new(
            server_runtime_http::tokio_child_process::TokioChildProcess::from(child),
            server_runtime_http::child_diagnostic_maximum_non_zero_usize::ChildDiagnosticMaximumNonZeroUsize::from(std::num::NonZeroUsize::MIN),
        );
            assert!(stdin.is_some());
            assert!(stdout_option.is_some());
            let Some(mut stdout) = stdout_option else {
                return;
            };
            if let Some(timeout) = request_timeout_duration {
                let mut shutdown = Box::pin(supervisor.shutdown(timeout));
                let pending = std::future::poll_fn(|context| {
                    std::task::Poll::Ready(shutdown.as_mut().poll(context).is_pending())
                })
                .await;
                assert!(pending);
                drop(shutdown);
            } else {
                drop(supervisor);
            }
            let mut buffer = [0u8; 1usize];
            let read_result = tokio::time::timeout(
                std::time::Duration::from_secs(60u64),
                tokio::io::AsyncReadExt::read(&mut stdout, &mut buffer),
            )
            .await;
            drop(stdin);
            assert!(read_result.is_ok());
            let Ok(completed_read) = read_result else {
                return;
            };
            assert!(completed_read.is_ok());
            let Ok(read) = completed_read else {
                return;
            };
            assert_eq!(read, 0usize);
        };
        check_drop(None).await;
        let timeout_result =
            server_runtime_http::request_timeout_duration::RequestTimeoutDuration::try_from(
                std::time::Duration::from_secs(60u64),
            );
        assert_ne!(timeout_result, Err(server_runtime_http::std_request_timeout_try_from_duration_error::StdRequestTimeoutTryFromDurationError::Zero));
        if let Ok(timeout) = timeout_result {
            check_drop(Some(timeout)).await;
        }
    }
    #[tokio::test]
    #[ignore = "requires true and false executables on PATH; run in a provisioned process environment"]
    async fn test_child_process_set_shutdown_preserves_insertion_order_and_status() {
        let create_process = |child_process_succeeded: server_runtime_http::child_process_succeeded::ChildProcessSucceeded| {
            let program = match child_process_succeeded {
                server_runtime_http::child_process_succeeded::ChildProcessSucceeded::Yes => constants_str::TRUE,
                server_runtime_http::child_process_succeeded::ChildProcessSucceeded::No => constants_str::FALSE,
            };
            tokio::process::Command::new(program).stderr(std::process::Stdio::null()).spawn()
                .map(|child| server_runtime_http::child_process_supervisor::ChildProcessSupervisor::new(
                    server_runtime_http::tokio_child_process::TokioChildProcess::from(child),
                    server_runtime_http::child_diagnostic_maximum_non_zero_usize::ChildDiagnosticMaximumNonZeroUsize::from(std::num::NonZeroUsize::MIN),
                ))
                .map_err(server_runtime_http::child_process_io_error::ChildProcessIoError::from)
        };
        let maximum_result = std::num::NonZeroUsize::try_from(2usize);
        assert!(maximum_result.is_ok_and(|maximum| maximum.get() == 2usize));
        let Ok(maximum) = maximum_result else {
            return;
        };
        let mut processes = server_runtime_http::child_process_set::ChildProcessSet::new(server_runtime_http::child_process_set_maximum_non_zero_usize::ChildProcessSetMaximumNonZeroUsize::from(maximum));
        let first_result =
            create_process(server_runtime_http::child_process_succeeded::ChildProcessSucceeded::No);
        assert!(first_result.is_ok());
        let Ok(first) = first_result else {
            return;
        };
        assert!(
            processes
                .insert(first)
                .is_ok_and(|child_process_id| child_process_id
                    == server_runtime_http::child_process_id::ChildProcessId::from(0u64))
        );
        let second_result = create_process(
            server_runtime_http::child_process_succeeded::ChildProcessSucceeded::Yes,
        );
        assert!(second_result.is_ok());
        let Ok(second) = second_result else {
            return;
        };
        assert!(
            processes
                .insert(second)
                .is_ok_and(|child_process_id| child_process_id
                    == server_runtime_http::child_process_id::ChildProcessId::from(1u64))
        );
        let timeout_result =
            server_runtime_http::request_timeout_duration::RequestTimeoutDuration::try_from(
                std::time::Duration::from_secs(60u64),
            );
        assert_ne!(timeout_result, Err(server_runtime_http::std_request_timeout_try_from_duration_error::StdRequestTimeoutTryFromDurationError::Zero));
        let Ok(timeout) = timeout_result else {
            return;
        };
        let reports_result = processes.shutdown_all(timeout).await;
        assert!(reports_result.is_ok());
        let Ok(reports) = reports_result else {
            return;
        };
        let statuses = reports
            .as_ref()
            .iter()
            .map(|report| report.status().succeeded())
            .collect::<Vec<_>>();
        assert_eq!(
            statuses,
            [
                server_runtime_http::child_process_succeeded::ChildProcessSucceeded::No,
                server_runtime_http::child_process_succeeded::ChildProcessSucceeded::Yes
            ]
        );
        assert!(reports.as_ref().iter().all(|report| report.completion()
            == server_runtime_http::child_process_completion::ChildProcessCompletion::Exited));
    }
}
