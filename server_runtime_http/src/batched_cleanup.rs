#[derive(optml::Optml, Clone, Copy, Debug, Eq, PartialEq)]
pub struct CleanupBatchSize(u64);

#[derive(optml::Optml, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
#[error("cleanup batch size must be greater than zero")]
pub struct CleanupBatchSizeError;

impl TryFrom<u64> for CleanupBatchSize {
    type Error = CleanupBatchSizeError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if value == 0u64 {
            Err(CleanupBatchSizeError)
        } else {
            Ok(Self(value))
        }
    }
}

#[derive(
    optml::Optml,
    Clone,
    Copy,
    Debug,
    Default,
    Eq,
    PartialEq,
    newtype::FromInner,
    newtype::IntoInnerFrom,
)]
pub struct CleanupRows(u64);

#[derive(
    optml::Optml,
    Clone,
    Copy,
    Debug,
    Default,
    Eq,
    PartialEq,
    newtype::FromInner,
    newtype::IntoInnerFrom,
)]
pub struct CleanupBatchCount(u64);

#[derive(optml::Optml, Clone, Copy, Debug, Eq, PartialEq)]
pub enum CleanupContinuation {
    Continue,
    Stop,
}

#[derive(optml::Optml, Clone, Copy, Debug, Eq, PartialEq)]
pub enum CleanupCompletion {
    Drained,
    Stopped,
}

#[derive(optml::Optml, Clone, Copy, Debug, Eq, PartialEq)]
#[allow(clippy::arbitrary_source_item_ordering)] // alignment order required by optml takes precedence over alphabetical field order
pub struct CleanupReport {
    batches: CleanupBatchCount,
    rows: CleanupRows,
    completion: CleanupCompletion,
}

impl CleanupReport {
    #[must_use]
    pub const fn batches(self) -> CleanupBatchCount {
        self.batches
    }

    #[must_use]
    pub const fn completion(self) -> CleanupCompletion {
        self.completion
    }

    #[must_use]
    pub const fn rows(self) -> CleanupRows {
        self.rows
    }
}

pub async fn run_batched_cleanup<Cleanup, CleanupFuture, CleanupError, Continue>(
    batch_size: CleanupBatchSize,
    mut cleanup: Cleanup,
    mut continuation: Continue,
) -> Result<CleanupReport, CleanupError>
where
    Cleanup: FnMut(CleanupBatchSize) -> CleanupFuture,
    CleanupFuture: Future<Output = Result<CleanupRows, CleanupError>>,
    Continue: FnMut() -> CleanupContinuation,
{
    let mut batches = 0u64;
    let mut rows = 0u64;
    loop {
        if continuation() == CleanupContinuation::Stop {
            return Ok(CleanupReport {
                batches: CleanupBatchCount::from(batches),
                completion: CleanupCompletion::Stopped,
                rows: CleanupRows::from(rows),
            });
        }
        let batch_rows = u64::from(cleanup(batch_size).await?);
        batches = batches.saturating_add(1u64);
        rows = rows.saturating_add(batch_rows);
        if batch_rows < batch_size.0 {
            return Ok(CleanupReport {
                batches: CleanupBatchCount::from(batches),
                completion: CleanupCompletion::Drained,
                rows: CleanupRows::from(rows),
            });
        }
    }
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn drains_full_batches_until_partial_batch() {
        let batches = [3u64, 3u64, 1u64];
        let batch_index = std::sync::atomic::AtomicUsize::new(0usize);
        let report = super::run_batched_cleanup(
            super::CleanupBatchSize::try_from(3u64)
                .expect("c3cfcb75 drains_full_batches_until_partial_batch invariant must hold"),
            async |_batch_size| {
                let index = batch_index.fetch_add(1usize, std::sync::atomic::Ordering::Relaxed);
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
        assert_eq!(u64::from(report.batches()), 0u64);
        assert_eq!(report.completion(), super::CleanupCompletion::Stopped);
    }

    #[test]
    fn zero_batch_size_is_rejected() {
        assert_eq!(
            super::CleanupBatchSize::try_from(0u64),
            Err(super::CleanupBatchSizeError)
        );
    }
}
