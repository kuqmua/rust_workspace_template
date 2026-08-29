pub use super::cleanup_batch_count::CleanupBatchCount;
pub use super::cleanup_batch_size::CleanupBatchSize;
pub use super::cleanup_batch_size_error::CleanupBatchSizeError;
pub use super::cleanup_completion::CleanupCompletion;
pub use super::cleanup_continuation::CleanupContinuation;
pub use super::cleanup_report::CleanupReport;
pub use super::cleanup_rows::CleanupRows;
pub use super::run_batched_cleanup::run_batched_cleanup;
#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn drains_full_batches_until_partial_batch() {
        let batches = [3u64, 3u64, 1u64];
        let batch_index = std::sync::atomic::AtomicUsize::new(constants_usize::ZERO);
        let report = super::run_batched_cleanup(
            super::CleanupBatchSize::try_from(3u64)
                .expect("c3cfcb75 drains_full_batches_until_partial_batch invariant must hold"),
            async |_batch_size| {
                let index = batch_index
                    .fetch_add(constants_usize::ONE, std::sync::atomic::Ordering::Relaxed);
                let rows = batches.get(index).copied().unwrap_or_default();
                Ok::<super::CleanupRows, std::convert::Infallible>(rows.into())
            },
            || super::CleanupContinuation::Continue,
        )
        .await
        .expect("8846789f drains_full_batches_until_partial_batch invariant must hold");
        assert_eq!(u64::from(report.batches()), 3u64);
        assert_eq!(u64::from(report.rows()), 7u64);
        assert_eq!(report.completion(), super::CleanupCompletion::Drained);
    }

    #[tokio::test]
    async fn cancellation_stops_before_query() {
        let report = super::run_batched_cleanup(
            super::CleanupBatchSize::try_from(3u64)
                .expect("116ff79d cancellation_stops_before_query invariant must hold"),
            async |_batch_size| Ok::<super::CleanupRows, std::convert::Infallible>(3u64.into()),
            || super::CleanupContinuation::Stop,
        )
        .await
        .expect("39247aa8 cancellation_stops_before_query invariant must hold");
        assert_eq!(u64::from(report.batches()), constants_u64::ZERO);
        assert_eq!(report.completion(), super::CleanupCompletion::Stopped);
    }

    #[test]
    fn zero_batch_size_is_rejected() {
        assert_eq!(
            super::CleanupBatchSize::try_from(constants_u64::ZERO),
            Err(super::CleanupBatchSizeError)
        );
    }
}

// Root-owned module compatibility wrappers.
mod cleanup_batch_count {
    pub use super::super::cleanup_batch_count::*;
}
mod cleanup_batch_size {
    pub use super::super::cleanup_batch_size::*;
}
mod cleanup_batch_size_error {
    pub use super::super::cleanup_batch_size_error::*;
}
mod cleanup_completion {
    pub use super::super::cleanup_completion::*;
}
mod cleanup_continuation {
    pub use super::super::cleanup_continuation::*;
}
mod cleanup_report {
    pub use super::super::cleanup_report::*;
}
mod cleanup_rows {
    pub use super::super::cleanup_rows::*;
}
mod run_batched_cleanup {
    pub use super::super::run_batched_cleanup::*;
}
