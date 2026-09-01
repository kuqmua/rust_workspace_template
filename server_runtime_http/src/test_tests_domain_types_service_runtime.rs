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

#[tokio::test]
async fn test_status_route_and_parts_are_stable() {
    let runtime = crate::service_runtime::ServiceRuntime::new(
        crate::add_status_route::add_status_route(crate::axum_router::AxumRouter::from(
            axum::Router::new(),
        )),
        None,
    );
    let (router, optional_task) = runtime.into_parts();
    assert!(optional_task.is_none());
    let response = tower::ServiceExt::oneshot(
        axum::Router::from(router),
        axum::extract::Request::builder()
            .uri(constants_str::STATUS)
            .body(axum::body::Body::empty())
            .expect(constants_str::DIAGNOSTIC_8E9C3DA1),
    )
    .await
    .expect(constants_str::DIAGNOSTIC_1E97AD3B);
    assert_eq!(response.status(), http::StatusCode::OK);
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
    let closed = crate::acquire_permit::acquire_permit(
        crate::arc_tokio_semaphore::ArcTokioSemaphore::from(semaphore),
        crate::permit_wait_timeout_duration::PermitWaitTimeoutDuration::from(
            std::time::Duration::ZERO,
        ),
        retry_after,
    )
    .await;
    assert!(matches!(
        closed,
        Err(crate::acquire_permit_error::AcquirePermitError::Closed(_))
    ));
    drop(closed);
    assert_eq!(
        http::HeaderValue::try_from(retry_after).expect(constants_str::DIAGNOSTIC_CB2A239C),
        http::HeaderValue::from_static("3")
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
    assert!(semaphore.try_acquire().is_some());
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
