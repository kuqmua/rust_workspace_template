#[tokio::test]
async fn test_async_run_history_keeps_latest_reports() {
    let history = server_runtime_core::async_run_history::AsyncRunHistory::new(
        server_runtime_core::async_run_history_maximum_len_non_zero_usize::AsyncRunHistoryMaximumLenNonZeroUsize::try_from(2usize)
            .expect(constants_str::DIAGNOSTIC_8567A9DF),
    );
    history.push(1u8).await;
    history.push(2u8).await;
    history.push(3u8).await;
    let snapshot = history.snapshot().await;
    assert_eq!(usize::from(snapshot.report_count()), 2usize);
    assert_eq!(snapshot.latest_report(), Some(&3u8));
}

#[test]
fn test_service_runtime_parts_are_stable() {
    let runtime = crate::service_runtime::ServiceRuntime::new(
        crate::axum_router::AxumRouter::from(axum::Router::new()),
        None,
    );
    let (router, optional_task) = runtime.into_parts();
    assert!(optional_task.is_none());
    drop(axum::Router::from(router));
    let optional_interval_task = crate::spawn_interval_task::spawn_interval_task(None, async || {});
    assert!(optional_interval_task.is_none());
}

#[tokio::test]
async fn test_background_task_shutdown_is_observable() {
    let interval = crate::run_interval_duration::RunIntervalDuration::try_from(
        std::time::Duration::from_secs(1u64),
    )
    .expect(constants_str::DIAGNOSTIC_E76640C4);
    let task = crate::spawn_interval_task::spawn_interval_task(Some(interval), async || {})
        .expect(constants_str::DIAGNOSTIC_32858863);
    let timeout = crate::request_timeout_duration::RequestTimeoutDuration::try_from(
        std::time::Duration::from_secs(1u64),
    )
    .expect(constants_str::DIAGNOSTIC_728B52B3);
    assert_eq!(
        task.shutdown(timeout)
            .await
            .expect(constants_str::DIAGNOSTIC_0D71D1B8),
        crate::background_task_outcome::BackgroundTaskOutcome::ShutdownRequested
    );
}

#[tokio::test]
async fn test_background_task_panic_is_observable() {
    let interval = crate::run_interval_duration::RunIntervalDuration::try_from(
        std::time::Duration::from_secs(1u64),
    )
    .expect(constants_str::DIAGNOSTIC_C9D73CAB);
    let task = crate::spawn_interval_task::spawn_interval_task(Some(interval), async || {
        std::panic::panic_any(constants_str::PANIC_62839854)
    })
    .expect(constants_str::DIAGNOSTIC_7A86A253);
    assert!(matches!(
        task.join().await,
        Err(crate::background_task_shutdown_error::BackgroundTaskShutdownError::Join(_))
    ));
}

#[tokio::test(start_paused = true)]
async fn test_cancelled_background_waits_abort_owned_tasks() {
    let assert_cancelled =
        async |option: Option<crate::request_timeout_duration::RequestTimeoutDuration>| {
            let (sender, receiver) = tokio::sync::oneshot::channel::<()>();
            let task_join = tokio::spawn(async move {
                let retained_sender = sender;
                let outcome = std::future::pending().await;
                drop(retained_sender);
                outcome
            });
            let task = crate::background_task::BackgroundTask::new(
                None,
                Some(crate::tokio_background_task_join::TokioBackgroundTaskJoin::from(task_join)),
            );
            if let Some(timeout) = option {
                let mut shutdown = std::pin::pin!(task.shutdown(timeout));
                std::future::poll_fn(|context| {
                    assert!(shutdown.as_mut().poll(context).is_pending());
                    std::task::Poll::Ready(())
                })
                .await;
            } else {
                let mut join = std::pin::pin!(task.join());
                std::future::poll_fn(|context| {
                    assert!(join.as_mut().poll(context).is_pending());
                    std::task::Poll::Ready(())
                })
                .await;
            }
            assert!(matches!(
                tokio::time::timeout(std::time::Duration::from_secs(1u64), receiver).await,
                Ok(Err(_))
            ));
        };
    assert_cancelled(None).await;
    let timeout = crate::request_timeout_duration::RequestTimeoutDuration::try_from(
        std::time::Duration::from_secs(1u64),
    )
    .expect(constants_str::DIAGNOSTIC_22D2BE20);
    assert_cancelled(Some(timeout)).await;
}

#[tokio::test(start_paused = true)]
async fn test_stuck_background_task_reaches_shutdown_timeout() {
    let interval = crate::run_interval_duration::RunIntervalDuration::try_from(
        std::time::Duration::from_secs(1u64),
    )
    .expect(constants_str::DIAGNOSTIC_F797718F);
    let task = crate::spawn_interval_task::spawn_interval_task(Some(interval), async || {
        std::future::pending::<()>().await;
    })
    .expect(constants_str::DIAGNOSTIC_A58F09DC);
    tokio::task::yield_now().await;
    let timeout = crate::request_timeout_duration::RequestTimeoutDuration::try_from(
        std::time::Duration::from_secs(1u64),
    )
    .expect(constants_str::DIAGNOSTIC_AE1262BB);
    let shutdown = tokio::spawn(task.shutdown(timeout));
    tokio::task::yield_now().await;
    tokio::time::advance(std::time::Duration::from_secs(1u64)).await;
    assert!(matches!(
        shutdown.await.expect(constants_str::DIAGNOSTIC_9E76A810),
        Err(crate::background_task_shutdown_error::BackgroundTaskShutdownError::Timeout)
    ));
}

#[tokio::test]
async fn test_acquire_permit_distinguishes_available_timeout_and_closed() {
    let retry_after = crate::retry_after_secs::RetryAfterSecs::try_from(3u64)
        .expect(constants_str::DIAGNOSTIC_C52D0E93);
    let semaphore = std::sync::Arc::new(tokio::sync::Semaphore::new(constants_usize::ONE));
    let permit = crate::acquire_permit::acquire_permit(
        crate::arc_tokio_semaphore::ArcTokioSemaphore::from(std::sync::Arc::clone(&semaphore)),
        crate::permit_wait_timeout_duration::PermitWaitTimeoutDuration::from(
            std::time::Duration::ZERO,
        ),
        retry_after,
    )
    .await
    .expect(constants_str::DIAGNOSTIC_E1394CD0);
    let timeout = crate::acquire_permit::acquire_permit(
        crate::arc_tokio_semaphore::ArcTokioSemaphore::from(std::sync::Arc::clone(&semaphore)),
        crate::permit_wait_timeout_duration::PermitWaitTimeoutDuration::from(
            std::time::Duration::ZERO,
        ),
        retry_after,
    )
    .await;
    assert!(matches!(
        timeout,
        Err(crate::acquire_permit_error::AcquirePermitError::Timeout(value)) if value == retry_after
    ));
    drop(timeout);
    drop(permit);
    semaphore.close();
    let expected = semaphore
        .acquire()
        .await
        .err()
        .map(|error| error.to_string());
    assert!(expected.is_some());
    let closed = crate::acquire_permit::acquire_permit(
        crate::arc_tokio_semaphore::ArcTokioSemaphore::from(semaphore),
        crate::permit_wait_timeout_duration::PermitWaitTimeoutDuration::from(
            std::time::Duration::ZERO,
        ),
        retry_after,
    )
    .await;
    assert!(closed.as_ref().is_err_and(|error| {
        matches!(
            error,
            crate::acquire_permit_error::AcquirePermitError::Closed(_)
        ) && std::error::Error::source(error).is_some_and(|source| {
            source.is::<crate::tokio_acquire_error::TokioAcquireError>()
                && Some(source.to_string()) == expected
        })
    }));
    drop(closed);
    assert_eq!(
        http::HeaderValue::try_from(retry_after).expect(constants_str::DIAGNOSTIC_CB2A239C),
        http::HeaderValue::from_static(constants_str::VALUE_4E074085)
    );
}

#[test]
fn test_concurrency_limit_wrappers_validate_boundaries_and_try_acquire() {
    assert_eq!(
        crate::retry_after_secs::RetryAfterSecs::try_from(constants_u64::ZERO),
        Err(crate::retry_after_secs_try_from_u64_error::RetryAfterSecsTryFromU64Error::Zero)
    );
    let permit_count = std::num::NonZeroUsize::new(constants_usize::ONE)
        .expect(constants_str::DIAGNOSTIC_50A95013);
    let semaphore = crate::arc_tokio_semaphore::ArcTokioSemaphore::new(
        crate::semaphore_permit_count_non_zero_usize::SemaphorePermitCountNonZeroUsize::from(
            permit_count,
        ),
    );
    let permit = semaphore
        .try_acquire()
        .expect(constants_str::DIAGNOSTIC_626040D0);
    assert!(semaphore.try_acquire().is_none());
    drop(permit);
    assert!(
        semaphore
            .try_acquire()
            .is_some_and(|tokio_owned_semaphore_permit| {
                tokio_owned_semaphore_permit.forget();
                semaphore.try_acquire().is_none()
            })
    );
}

#[test]
fn test_zero_limits_are_rejected() {
    let Err(history_error) =
        server_runtime_core::async_run_history_maximum_len_non_zero_usize::AsyncRunHistoryMaximumLenNonZeroUsize::try_from(constants_usize::ZERO)
    else {
        std::panic::panic_any(constants_str::PANIC_5500CD77);
    };
    assert_eq!(
        history_error,
        server_runtime_core::std_async_run_history_maximum_len_try_from_usize_error::StdAsyncRunHistoryMaximumLenTryFromUsizeError::Zero
    );
    let Err(timeout_error) = crate::request_timeout_duration::RequestTimeoutDuration::try_from(
        std::time::Duration::ZERO,
    ) else {
        std::panic::panic_any(constants_str::PANIC_BCA83CB0);
    };
    assert_eq!(
        timeout_error,
        crate::std_request_timeout_try_from_duration_error::StdRequestTimeoutTryFromDurationError::Zero
    );
}

#[tokio::test(start_paused = true)]
async fn test_extreme_interval_reports_failure_without_running_callback() {
    let interval =
        crate::run_interval_duration::RunIntervalDuration::try_from(std::time::Duration::MAX)
            .expect(constants_str::DIAGNOSTIC_A104298F);
    let task = crate::spawn_interval_task::spawn_interval_task(Some(interval), async || {
        std::future::pending::<()>().await;
    })
    .expect(constants_str::DIAGNOSTIC_7CE0ABD6);
    assert!(matches!(
        tokio::time::timeout(std::time::Duration::from_secs(1u64), task.join()).await,
        Ok(Err(
            crate::background_task_shutdown_error::BackgroundTaskShutdownError::IntervalOverflow
        ))
    ));
}

async fn assert_interval_skip_cadence(
    run_interval_duration: crate::run_interval_duration::RunIntervalDuration,
) {
    let period = run_interval_duration.get();
    let margin = period
        .checked_div(100u32)
        .expect(constants_str::DIAGNOSTIC_8204AC59);
    let half_margin = margin
        .checked_div(2u32)
        .expect(constants_str::DIAGNOSTIC_7EAC7AE5);
    let elapsed = period
        .checked_mul(4u32)
        .and_then(|total| total.checked_sub(margin))
        .expect(constants_str::DIAGNOSTIC_5A95A59C);
    let (sender, mut receiver) = tokio::sync::mpsc::channel(4usize);
    let optional_task =
        crate::spawn_interval_task::spawn_interval_task(Some(run_interval_duration), move || {
            assert!(matches!(sender.try_send(()), Ok(())));
            async {}
        });
    assert!(matches!(
        tokio::time::timeout(std::time::Duration::from_millis(1u64), receiver.recv()).await,
        Ok(Some(()))
    ));
    tokio::time::advance(elapsed).await;
    assert!(matches!(
        tokio::time::timeout(std::time::Duration::from_millis(1u64), receiver.recv()).await,
        Ok(Some(()))
    ));
    assert!(matches!(
        receiver.try_recv(),
        Err(tokio::sync::mpsc::error::TryRecvError::Empty)
    ));
    tokio::time::advance(half_margin).await;
    assert!(matches!(
        receiver.try_recv(),
        Err(tokio::sync::mpsc::error::TryRecvError::Empty)
    ));
    tokio::time::advance(half_margin).await;
    assert!(matches!(
        tokio::time::timeout(std::time::Duration::from_millis(1u64), receiver.recv()).await,
        Ok(Some(()))
    ));
    let Some(task) = optional_task else {
        assert!(run_interval_duration.get().is_zero());
        return;
    };
    let timeout = crate::request_timeout_duration::RequestTimeoutDuration::try_from(
        std::time::Duration::from_secs(1u64),
    );
    assert!(matches!(timeout, Ok(request_timeout_duration)
        if matches!(task.shutdown(request_timeout_duration).await,
            Ok(crate::background_task_outcome::BackgroundTaskOutcome::ShutdownRequested))));
}

#[tokio::test(start_paused = true)]
async fn test_interval_skip_keeps_original_cadence() {
    let interval = crate::run_interval_duration::RunIntervalDuration::try_from(
        std::time::Duration::from_millis(10u64),
    )
    .expect(constants_str::DIAGNOSTIC_E1C936F5);
    assert_interval_skip_cadence(interval).await;
}

#[cfg(target_os = "linux")]
#[tokio::test(start_paused = true)]
async fn test_interval_skip_handles_remainders_above_u64_nanoseconds() {
    let interval = crate::run_interval_duration::RunIntervalDuration::try_from(
        std::time::Duration::from_hours(5_256_000u64),
    )
    .expect(constants_str::DIAGNOSTIC_27887819);
    assert_interval_skip_cadence(interval).await;
}

#[test]
fn test_run_interval_duration_rejects_zero_and_preserves_positive_precision() {
    assert_eq!(
        crate::run_interval_duration::RunIntervalDuration::try_from(std::time::Duration::ZERO),
        Err(crate::std_run_interval_try_from_duration_error::StdRunIntervalTryFromDurationError::Zero),
    );
    assert!(
        [
            std::time::Duration::from_nanos(1u64),
            std::time::Duration::new(1u64, 999_999_999u32),
            std::time::Duration::MAX,
        ]
        .into_iter()
        .all(|duration| {
            crate::run_interval_duration::RunIntervalDuration::try_from(duration)
                .is_ok_and(|interval| interval.get() == duration)
        })
    );
}

#[test]
fn test_retry_after_header_preserves_full_nonzero_u64_range() {
    assert!([1u64, u64::MAX - 1u64, u64::MAX].into_iter().all(|value| {
        crate::retry_after_secs::RetryAfterSecs::try_from(value).is_ok_and(|retry| {
            retry.get() == value
                && http::HeaderValue::try_from(retry)
                    .is_ok_and(|header| header.as_bytes() == value.to_string().as_bytes())
        })
    }));
}
