#[tokio::test]
async fn async_run_history_keeps_latest_reports() {
    let history = crate::AsyncRunHistory::new(
        crate::AsyncRunHistoryMaximumLenNonZeroUsize::try_from(2usize)
            .expect("8567a9df async_run_history_keeps_latest_reports invariant must hold"),
    );
    history.push(1u8).await;
    history.push(2u8).await;
    history.push(3u8).await;
    let snapshot = history.snapshot().await;
    assert_eq!(usize::from(snapshot.report_count()), 2usize);
    assert_eq!(snapshot.latest_report(), Some(&3u8));
}

#[tokio::test]
async fn status_route_and_parts_are_stable() {
    let runtime = crate::ServiceRuntime::new(
        crate::add_status_route(crate::AxumRouter::from(axum::Router::new())),
        None,
    );
    let (router, optional_task) = runtime.into_parts();
    assert!(optional_task.is_none());
    let response = tower::ServiceExt::oneshot(
        axum::Router::from(router),
        axum::extract::Request::builder()
            .uri(constants_str::STATUS)
            .body(axum::body::Body::empty())
            .expect("8e9c3da1 status_route_and_parts_are_stable invariant must hold"),
    )
    .await
    .expect("1e97ad3b status_route_and_parts_are_stable invariant must hold");
    assert_eq!(response.status(), http::StatusCode::OK);
    let optional_interval_task = crate::spawn_interval_task(None, async || {});
    assert!(optional_interval_task.is_none());
}

#[tokio::test]
async fn background_task_shutdown_is_observable() {
    let interval = crate::RunIntervalDuration::try_from(std::time::Duration::from_secs(1u64))
        .expect("e76640c4 background_task_shutdown_is_observable invariant must hold");
    let task = crate::spawn_interval_task(Some(interval), async || {})
        .expect("32858863 background_task_shutdown_is_observable invariant must hold");
    let timeout = crate::RequestTimeoutDuration::try_from(std::time::Duration::from_secs(1u64))
        .expect("728b52b3 background_task_shutdown_is_observable invariant must hold");
    assert_eq!(
        task.shutdown(timeout)
            .await
            .expect("0d71d1b8 background_task_shutdown_is_observable invariant must hold"),
        crate::BackgroundTaskOutcome::ShutdownRequested
    );
}

#[tokio::test]
async fn background_task_panic_is_observable() {
    let interval = crate::RunIntervalDuration::try_from(std::time::Duration::from_secs(1u64))
        .expect("c9d73cab background_task_panic_is_observable invariant must hold");
    let task = crate::spawn_interval_task(Some(interval), async || panic!("62839854"))
        .expect("7a86a253 background_task_panic_is_observable invariant must hold");
    assert!(matches!(
        task.join().await,
        Err(crate::BackgroundTaskShutdownError::Join(_))
    ));
}

#[tokio::test(start_paused = true)]
async fn stuck_background_task_reaches_shutdown_timeout() {
    let interval = crate::RunIntervalDuration::try_from(std::time::Duration::from_secs(1u64))
        .expect("f797718f stuck_background_task_reaches_shutdown_timeout invariant must hold");
    let task = crate::spawn_interval_task(Some(interval), async || {
        std::future::pending::<()>().await;
    })
    .expect("a58f09dc stuck_background_task_reaches_shutdown_timeout invariant must hold");
    tokio::task::yield_now().await;
    let timeout = crate::RequestTimeoutDuration::try_from(std::time::Duration::from_secs(1u64))
        .expect("ae1262bb stuck_background_task_reaches_shutdown_timeout invariant must hold");
    let shutdown = tokio::spawn(task.shutdown(timeout));
    tokio::task::yield_now().await;
    tokio::time::advance(std::time::Duration::from_secs(1u64)).await;
    assert!(matches!(
        shutdown
            .await
            .expect("9e76a810 stuck_background_task_reaches_shutdown_timeout invariant must hold"),
        Err(crate::BackgroundTaskShutdownError::Timeout)
    ));
}

#[tokio::test]
async fn acquire_permit_distinguishes_available_timeout_and_closed() {
    let retry_after = crate::RetryAfterSecs::try_from(3u64).expect(
        "c52d0e93 acquire_permit_distinguishes_available_timeout_and_closed invariant must hold",
    );
    let semaphore = std::sync::Arc::new(tokio::sync::Semaphore::new(constants_usize::ONE));
    let permit = crate::acquire_permit(
        crate::ArcTokioSemaphore::from(std::sync::Arc::clone(&semaphore)),
        crate::PermitWaitTimeoutDuration::from(std::time::Duration::ZERO),
        retry_after,
    )
    .await
    .expect(
        "e1394cd0 acquire_permit_distinguishes_available_timeout_and_closed invariant must hold",
    );
    let timeout = crate::acquire_permit(
        crate::ArcTokioSemaphore::from(std::sync::Arc::clone(&semaphore)),
        crate::PermitWaitTimeoutDuration::from(std::time::Duration::ZERO),
        retry_after,
    )
    .await;
    assert!(matches!(
        timeout,
        Err(crate::AcquirePermitError::Timeout(value)) if value == retry_after
    ));
    drop(timeout);
    drop(permit);
    semaphore.close();
    let closed = crate::acquire_permit(
        crate::ArcTokioSemaphore::from(semaphore),
        crate::PermitWaitTimeoutDuration::from(std::time::Duration::ZERO),
        retry_after,
    )
    .await;
    assert!(matches!(closed, Err(crate::AcquirePermitError::Closed(_))));
    drop(closed);
    assert_eq!(
        http::HeaderValue::try_from(retry_after).expect(
            "cb2a239c acquire_permit_distinguishes_available_timeout_and_closed invariant must hold"
        ),
        http::HeaderValue::from_static("3")
    );
}

#[test]
fn concurrency_limit_wrappers_validate_boundaries_and_try_acquire() {
    assert_eq!(
        crate::RetryAfterSecs::try_from(constants_u64::ZERO),
        Err(crate::RetryAfterSecsTryFromU64Error)
    );
    let permit_count = std::num::NonZeroUsize::new(constants_usize::ONE).expect("50a95013 concurrency_limit_wrappers_validate_boundaries_and_try_acquire invariant must hold");
    let semaphore =
        crate::ArcTokioSemaphore::new(crate::SemaphorePermitCountNonZeroUsize::from(permit_count));
    let permit = semaphore.try_acquire().expect("626040d0 concurrency_limit_wrappers_validate_boundaries_and_try_acquire invariant must hold");
    assert!(semaphore.try_acquire().is_none());
    drop(permit);
    assert!(semaphore.try_acquire().is_some());
}

#[test]
fn zero_limits_are_rejected() {
    let Err(history_error) =
        crate::AsyncRunHistoryMaximumLenNonZeroUsize::try_from(constants_usize::ZERO)
    else {
        panic!("5500cd77");
    };
    assert_eq!(
        history_error,
        crate::StdAsyncRunHistoryMaximumLenTryFromUsizeError
    );
    let Err(timeout_error) = crate::RequestTimeoutDuration::try_from(std::time::Duration::ZERO)
    else {
        panic!("bca83cb0");
    };
    assert_eq!(timeout_error, crate::StdRequestTimeoutTryFromDurationError);
}
