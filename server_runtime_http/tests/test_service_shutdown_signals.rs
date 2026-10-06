#![cfg(unix)]
#![allow(
    unused_crate_dependencies,
    reason = "this integration target exercises the isolated Unix signal waiter and process lifecycle"
)]

#[cfg(test)]
mod tests {
    #[test]
    #[ignore = "invoked only as an isolated child by the provisioned shutdown signal test"]
    fn test_signal_waiter_child() {
        if !std::env::var(constants_str::SHUTDOWN_SIGNAL_TEST_CHILD_ENV_KEY)
            .is_ok_and(|value| value == constants_str::TRUE)
        {
            return;
        }
        let runtime_result = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build();
        assert!(runtime_result.is_ok());
        if let Ok(runtime) = runtime_result {
            let mut shutdown = std::pin::pin!(
                server_runtime_http::wait_for_service_shutdown_signal::wait_for_service_shutdown_signal()
            );
            let registered = runtime.block_on(std::future::poll_fn(|context| {
                std::task::Poll::Ready(shutdown.as_mut().poll(context).is_pending())
            }));
            assert!(registered);
            let readiness_result = {
                let mut stdout = std::io::stdout().lock();
                std::io::Write::write_all(&mut stdout, &[0u8])
                    .and_then(|()| std::io::Write::flush(&mut stdout))
            };
            assert!(matches!(&readiness_result, Ok(())));
            let shutdown_result = runtime.block_on(shutdown.as_mut());
            assert!(matches!(&shutdown_result, Ok(())));
        }
    }

    #[test]
    #[ignore = "requires a Unix kill utility and isolated child processes; run in a provisioned process environment"]
    fn test_service_shutdown_waiter_completes_after_interrupt_and_terminate() {
        let executable_result = std::env::current_exe();
        assert!(executable_result.is_ok());
        let runtime_result = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build();
        assert!(runtime_result.is_ok());
        if let (Ok(executable), Ok(runtime)) = (executable_result, runtime_result) {
            runtime.block_on(async {
                let check_signal = async |signal_argument| {
                    let child_result = tokio::process::Command::new(&executable)
                        .args([
                            constants_str::SOURCE_PLACE_TEST_EXACT_ARGUMENT,
                            constants_str::SHARED_VALUES_IGNORED,
                            constants_str::SHUTDOWN_SIGNAL_TEST_NOCAPTURE_ARGUMENT,
                            constants_str::SHUTDOWN_SIGNAL_TEST_CHILD_FILTER,
                        ])
                        .env(constants_str::SHUTDOWN_SIGNAL_TEST_CHILD_ENV_KEY, constants_str::TRUE)
                        .stdout(std::process::Stdio::piped())
                        .stderr(std::process::Stdio::inherit())
                        .kill_on_drop(true)
                        .spawn();
                    assert!(child_result.is_ok());
                    if let Ok(mut child) = child_result {
                        let outcome = tokio::time::timeout(std::time::Duration::from_secs(60u64), async {
                            let Some(stdout) = child.stdout.take() else {
                                return Ok(server_runtime_http::child_process_succeeded::ChildProcessSucceeded::No);
                            };
                            let mut reader = tokio::io::BufReader::new(tokio::io::AsyncReadExt::take(stdout, 1024u64));
                            let mut readiness = Vec::new();
                            let read_count = tokio::io::AsyncBufReadExt::read_until(&mut reader, 0u8, &mut readiness)
                                .await.map_err(server_runtime_http::service_runtime_io_error::ServiceRuntimeIoError::from)?;
                            if read_count == 0usize || readiness.last() != Some(&0u8) {
                                return Ok(server_runtime_http::child_process_succeeded::ChildProcessSucceeded::No);
                            }
                            let Some(identifier) = child.id() else {
                                return Ok(server_runtime_http::child_process_succeeded::ChildProcessSucceeded::No);
                            };
                            let delivery_status = tokio::process::Command::new(constants_str::SHUTDOWN_SIGNAL_TEST_KILL_PROGRAM)
                                .args([signal_argument, identifier.to_string().as_str()])
                                .kill_on_drop(true).status().await
                                .map_err(server_runtime_http::service_runtime_io_error::ServiceRuntimeIoError::from)?;
                            if !delivery_status.success() {
                                return Ok(server_runtime_http::child_process_succeeded::ChildProcessSucceeded::No);
                            }
                            child.wait().await
                                .map_err(server_runtime_http::service_runtime_io_error::ServiceRuntimeIoError::from)
                                .map(|status| if status.success() {
                                    server_runtime_http::child_process_succeeded::ChildProcessSucceeded::Yes
                                } else {
                                    server_runtime_http::child_process_succeeded::ChildProcessSucceeded::No
                                })
                        }).await;
                        let kill_result = child.start_kill();
                        let reap_result = tokio::time::timeout(std::time::Duration::from_secs(60u64), child.wait()).await;
                        assert!(matches!(&kill_result, Ok(())));
                        assert!(matches!(&reap_result, Ok(Ok(_))));
                        assert!(matches!(&outcome, Ok(Ok(server_runtime_http::child_process_succeeded::ChildProcessSucceeded::Yes))));
                    }
                };
                check_signal(constants_str::SHUTDOWN_SIGNAL_TEST_INTERRUPT_ARGUMENT).await;
                check_signal(constants_str::SHUTDOWN_SIGNAL_TEST_TERMINATE_ARGUMENT).await;
            });
        }
    }
}
