pub async fn run_batched_cleanup<Cleanup, CleanupFuture, CleanupError, Continue>(
    batch_size: super::CleanupBatchSize,
    mut cleanup: Cleanup,
    mut continuation: Continue,
) -> Result<super::CleanupReport, CleanupError>
where
    Cleanup: FnMut(super::CleanupBatchSize) -> CleanupFuture,
    CleanupFuture: Future<Output = Result<super::CleanupRows, CleanupError>>,
    Continue: FnMut() -> super::CleanupContinuation,
{
    let mut batches = constants_u64::ZERO;
    let mut rows = constants_u64::ZERO;
    loop {
        if continuation() == super::CleanupContinuation::Stop {
            return Ok(super::CleanupReport {
                batches: super::CleanupBatchCount::from(batches),
                completion: super::CleanupCompletion::Stopped,
                rows: super::CleanupRows::from(rows),
            });
        }
        let batch_rows = u64::from(cleanup(batch_size).await?);
        batches = batches.saturating_add(constants_u64::ONE);
        rows = rows.saturating_add(batch_rows);
        if batch_rows < batch_size.0.0.get() {
            return Ok(super::CleanupReport {
                batches: super::CleanupBatchCount::from(batches),
                completion: super::CleanupCompletion::Drained,
                rows: super::CleanupRows::from(rows),
            });
        }
    }
}
