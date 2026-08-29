#[must_use]
// The owner module retains lint-sensitive semantics from the original implementation.
#[allow(clippy::integer_division_remainder_used)]
pub fn spawn_interval_task<Run, RunFuture>(
    optional_interval: Option<crate::run_interval_duration::RunIntervalDuration>,
    mut run: Run,
) -> Option<crate::background_task::BackgroundTask>
where
    Run: FnMut() -> RunFuture + Send + 'static,
    RunFuture: Future<Output = ()> + Send + 'static,
{
    let interval = optional_interval?;
    let (shutdown_tx, mut shutdown_rx) = tokio::sync::oneshot::channel();
    let task_join = tokio::spawn(async move {
        let mut timer = tokio::time::interval(interval.0);
        timer.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);
        loop {
            tokio::select! {
                _shutdown_result = &mut shutdown_rx => {
                    return crate::background_task_outcome::BackgroundTaskOutcome::ShutdownRequested;
                }
                _tick = timer.tick() => run().await,
            }
        }
    });
    Some(crate::background_task::BackgroundTask {
        task_join: Some(
            crate::tokio_background_task_join::TokioBackgroundTaskJoin::from(task_join),
        ),
        shutdown_tx: Some(
            crate::tokio_background_task_shutdown_sender::TokioBackgroundTaskShutdownSender::from(
                shutdown_tx,
            ),
        ),
    })
}
