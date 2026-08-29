pub async fn run_batched_cleanup<Cleanup, CleanupFuture, CleanupError, Continue>(
    batch_size: crate::cleanup_batch_size::CleanupBatchSize,
    mut cleanup: Cleanup,
    mut continuation: Continue,
) -> Result<crate::cleanup_report::CleanupReport, CleanupError>
where
    Cleanup: FnMut(crate::cleanup_batch_size::CleanupBatchSize) -> CleanupFuture,
    CleanupFuture: Future<Output = Result<crate::cleanup_rows::CleanupRows, CleanupError>>,
    Continue: FnMut() -> crate::cleanup_continuation::CleanupContinuation,
{
    let mut batches = constants_u64::ZERO;
    let mut rows = constants_u64::ZERO;
    loop {
        if continuation() == crate::cleanup_continuation::CleanupContinuation::Stop {
            return Ok(crate::cleanup_report::CleanupReport {
                batches: crate::cleanup_batch_count::CleanupBatchCount::from(batches),
                completion: crate::cleanup_completion::CleanupCompletion::Stopped,
                rows: crate::cleanup_rows::CleanupRows::from(rows),
            });
        }
        let batch_rows = u64::from(cleanup(batch_size).await?);
        batches = batches.saturating_add(constants_u64::ONE);
        rows = rows.saturating_add(batch_rows);
        if batch_rows < batch_size.0.get() {
            return Ok(crate::cleanup_report::CleanupReport {
                batches: crate::cleanup_batch_count::CleanupBatchCount::from(batches),
                completion: crate::cleanup_completion::CleanupCompletion::Drained,
                rows: crate::cleanup_rows::CleanupRows::from(rows),
            });
        }
    }
}
