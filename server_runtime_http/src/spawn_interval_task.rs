#[must_use]
#[allow(
    clippy::integer_division_remainder_used,
    reason = "interval scheduling halves exact binary duration multiples when computing a remainder without narrowing nanoseconds or allocating"
)]
pub fn spawn_interval_task<Run, RunFuture>(
    option: Option<crate::run_interval_duration::RunIntervalDuration>,
    mut run: Run,
) -> Option<crate::background_task::BackgroundTask>
where
    Run: FnMut() -> RunFuture + Send + 'static,
    RunFuture: Future<Output = ()> + Send + 'static,
{
    let interval = option?;
    let (shutdown_tx, mut shutdown_rx) = tokio::sync::oneshot::channel();
    let task_join = tokio::spawn(async move {
        let mut deadline = tokio::time::Instant::now();
        loop {
            tokio::select! {
                _shutdown_result = &mut shutdown_rx => {
                    return crate::background_task_outcome::BackgroundTaskOutcome::ShutdownRequested;
                }
                _tick = tokio::time::sleep_until(deadline) => {
                    let now = tokio::time::Instant::now();
                    let missed = deadline.checked_add(std::time::Duration::from_millis(5u64))
                        .is_some_and(|threshold| now > threshold);
                    let next = if missed {
                        let mut remainder = now.duration_since(deadline);
                        let mut divisor = interval.get();
                        while let Some(doubled) = divisor.checked_mul(2u32) {
                            if doubled > remainder {
                                break;
                            }
                            divisor = doubled;
                        }
                        loop {
                            if remainder >= divisor {
                                remainder = remainder.saturating_sub(divisor);
                            }
                            if divisor == interval.get() {
                                break;
                            }
                            let Some(halved) = divisor.checked_div(2u32) else {
                                return crate::background_task_outcome::BackgroundTaskOutcome::IntervalOverflow;
                            };
                            divisor = halved;
                        }
                        let Some(delay) = interval.get().checked_sub(remainder) else {
                            return crate::background_task_outcome::BackgroundTaskOutcome::IntervalOverflow;
                        };
                        now.checked_add(delay)
                    } else {
                        deadline.checked_add(interval.get())
                    };
                    let Some(next_deadline) = next else {
                        return crate::background_task_outcome::BackgroundTaskOutcome::IntervalOverflow;
                    };
                    deadline = next_deadline;
                    run().await;
                },
            }
        }
    });
    Some(crate::background_task::BackgroundTask::new(
        Some(
            crate::tokio_background_task_shutdown_sender::TokioBackgroundTaskShutdownSender::from(
                shutdown_tx,
            ),
        ),
        Some(crate::tokio_background_task_join::TokioBackgroundTaskJoin::from(task_join)),
    ))
}
