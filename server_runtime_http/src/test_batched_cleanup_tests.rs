#[cfg(test)]
mod tests {
    #[test]
    fn test_dropping_cleanup_with_pending_callback_prevents_further_calls() {
        let cleanup_calls = std::cell::Cell::new(0u64);
        let continuation_calls = std::cell::Cell::new(0u64);
        let pending_without_reentry = {
            let mut cleanup = std::pin::pin!(crate::run_batched_cleanup::run_batched_cleanup(
                crate::cleanup_batch_size::CleanupBatchSize::from(std::num::NonZeroU64::MIN),
                async |cleanup_batch_size: crate::cleanup_batch_size::CleanupBatchSize| {
                    assert_eq!(cleanup_batch_size.get(), 1u64);
                    cleanup_calls.set(cleanup_calls.get().saturating_add(1u64));
                    std::future::pending::<
                        Result<crate::cleanup_rows::CleanupRows, std::convert::Infallible>,
                    >()
                    .await
                },
                || {
                    continuation_calls.set(continuation_calls.get().saturating_add(1u64));
                    crate::cleanup_continuation::CleanupContinuation::Continue
                },
            ));
            assert_eq!(cleanup_calls.get(), 0u64);
            assert_eq!(continuation_calls.get(), 0u64);
            let mut context = std::task::Context::from_waker(std::task::Waker::noop());
            (0u8..2u8).all(|_| {
                let poll = Future::poll(cleanup.as_mut(), &mut context);
                assert_eq!(cleanup_calls.get(), 1u64);
                assert_eq!(continuation_calls.get(), 1u64);
                poll.is_pending()
            })
        };
        assert!(pending_without_reentry);
        assert_eq!(cleanup_calls.get(), 1u64);
        assert_eq!(continuation_calls.get(), 1u64);
    }

    #[tokio::test]
    async fn test_cleanup_empty_first_batch_and_saturating_row_totals() {
        let run = async |cleanup_rows: crate::cleanup_rows::CleanupRows| {
            let calls = std::cell::Cell::new(0u64);
            let result = crate::run_batched_cleanup::run_batched_cleanup(
                crate::cleanup_batch_size::CleanupBatchSize::from(std::num::NonZeroU64::MAX),
                |_cleanup_batch_size: crate::cleanup_batch_size::CleanupBatchSize| {
                    let previous = calls.get();
                    calls.set(previous.saturating_add(1u64));
                    std::future::ready(Ok::<_, std::convert::Infallible>(if previous < 2u64 {
                        cleanup_rows
                    } else {
                        crate::cleanup_rows::CleanupRows::default()
                    }))
                },
                || crate::cleanup_continuation::CleanupContinuation::Continue,
            )
            .await;
            let expected_batches = if u64::from(cleanup_rows) == 0u64 {
                1u64
            } else {
                3u64
            };
            result.is_ok_and(|report| {
                u64::from(report.batches()) == expected_batches
                    && u64::from(report.rows()) == u64::from(cleanup_rows)
                    && report.completion() == crate::cleanup_completion::CleanupCompletion::Drained
            }) && calls.get() == expected_batches
        };
        assert!(run(crate::cleanup_rows::CleanupRows::default()).await);
        assert!(run(crate::cleanup_rows::CleanupRows::from(u64::MAX)).await);
    }

    #[tokio::test]
    async fn test_cleanup_stop_after_full_batch_preserves_counts() {
        let cleanup_calls = std::cell::Cell::new(0u64);
        let continuation_calls = std::cell::Cell::new(0u64);
        let result = crate::run_batched_cleanup::run_batched_cleanup(
            crate::cleanup_batch_size::CleanupBatchSize::from(std::num::NonZeroU64::MIN),
            |cleanup_batch_size: crate::cleanup_batch_size::CleanupBatchSize| {
                assert_eq!(cleanup_batch_size.get(), 1u64);
                cleanup_calls.set(cleanup_calls.get().saturating_add(1u64));
                std::future::ready(Ok::<_, std::convert::Infallible>(
                    crate::cleanup_rows::CleanupRows::from(1u64),
                ))
            },
            || {
                let previous = continuation_calls.get();
                continuation_calls.set(previous.saturating_add(1u64));
                if previous == 0u64 {
                    crate::cleanup_continuation::CleanupContinuation::Continue
                } else {
                    crate::cleanup_continuation::CleanupContinuation::Stop
                }
            },
        )
        .await;
        assert!(
            result.is_ok_and(|report| u64::from(report.batches()) == 1u64
                && u64::from(report.rows()) == 1u64
                && report.completion() == crate::cleanup_completion::CleanupCompletion::Stopped)
        );
        assert_eq!(cleanup_calls.get(), 1u64);
        assert_eq!(continuation_calls.get(), 2u64);
    }

    #[tokio::test]
    async fn test_cleanup_preserves_callback_error_without_retry() {
        let cleanup_calls = std::cell::Cell::new(0u64);
        let result = crate::run_batched_cleanup::run_batched_cleanup(
            crate::cleanup_batch_size::CleanupBatchSize::from(std::num::NonZeroU64::MIN),
            |cleanup_batch_size: crate::cleanup_batch_size::CleanupBatchSize| {
                assert_eq!(cleanup_batch_size.get(), 1u64);
                let previous = cleanup_calls.get();
                cleanup_calls.set(previous.saturating_add(1u64));
                std::future::ready(if previous == 0u64 {
                    Ok(crate::cleanup_rows::CleanupRows::from(1u64))
                } else {
                    Err(crate::cleanup_batch_size_error::CleanupBatchSizeError::Zero)
                })
            },
            || crate::cleanup_continuation::CleanupContinuation::Continue,
        )
        .await;
        assert_eq!(
            result,
            Err(crate::cleanup_batch_size_error::CleanupBatchSizeError::Zero)
        );
        assert_eq!(cleanup_calls.get(), 2u64);
    }

    #[tokio::test]
    async fn test_drains_full_batches_until_partial_batch() {
        let batches = [3u64, 3u64, 1u64];
        let batch_index = std::sync::atomic::AtomicUsize::new(constants_usize::ZERO);
        let report = crate::run_batched_cleanup::run_batched_cleanup(
            crate::cleanup_batch_size::CleanupBatchSize::try_from(3u64)
                .expect(constants_str::DIAGNOSTIC_C3CFCB75),
            async |_batch_size| {
                let index = batch_index
                    .fetch_add(constants_usize::ONE, std::sync::atomic::Ordering::Relaxed);
                let rows = batches.get(index).copied().unwrap_or_default();
                Ok::<crate::cleanup_rows::CleanupRows, std::convert::Infallible>(rows.into())
            },
            || crate::cleanup_continuation::CleanupContinuation::Continue,
        )
        .await
        .expect(constants_str::DIAGNOSTIC_8846789F);
        assert_eq!(u64::from(report.batches()), 3u64);
        assert_eq!(u64::from(report.rows()), 7u64);
        assert_eq!(
            report.completion(),
            crate::cleanup_completion::CleanupCompletion::Drained
        );
    }

    #[tokio::test]
    async fn test_cancellation_stops_before_query() {
        let report = crate::run_batched_cleanup::run_batched_cleanup(
            crate::cleanup_batch_size::CleanupBatchSize::try_from(3u64)
                .expect(constants_str::DIAGNOSTIC_116FF79D),
            async |_batch_size| {
                Ok::<crate::cleanup_rows::CleanupRows, std::convert::Infallible>(3u64.into())
            },
            || crate::cleanup_continuation::CleanupContinuation::Stop,
        )
        .await
        .expect(constants_str::DIAGNOSTIC_39247AA8);
        assert_eq!(u64::from(report.batches()), constants_u64::ZERO);
        assert_eq!(
            report.completion(),
            crate::cleanup_completion::CleanupCompletion::Stopped
        );
    }

    #[test]
    fn test_zero_batch_size_is_rejected() {
        assert_eq!(
            crate::cleanup_batch_size::CleanupBatchSize::try_from(constants_u64::ZERO),
            Err(crate::cleanup_batch_size_error::CleanupBatchSizeError::Zero)
        );
    }
}
