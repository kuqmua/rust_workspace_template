#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq)]
pub enum BackgroundTaskOutcome {
    Completed,
    ShutdownRequested,
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error, newtype::FromInner,
)]
#[error(transparent)]
pub struct TokioTaskJoinError(tokio::task::JoinError);
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, newtype::FromInner)]
pub struct TokioAbortTask(tokio::task::JoinHandle<()>);

#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
pub enum BackgroundTaskShutdownError {
    #[error("background task failed: {0}")]
    Join(#[source] TokioTaskJoinError),
    #[error("{}", constants_str::BACKGROUND_TASK_SHUTDOWN_TIMED_OUT)]
    Timeout,
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug)]
#[must_use]
pub struct BackgroundTask {
    shutdown_tx: Option<TokioBackgroundTaskShutdownSender>,
    task_join: Option<TokioBackgroundTaskJoin>,
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, newtype::FromInner)]
struct TokioBackgroundTaskJoin(tokio::task::JoinHandle<BackgroundTaskOutcome>);

#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, newtype::FromInner)]
struct TokioBackgroundTaskShutdownSender(tokio::sync::oneshot::Sender<()>);

impl BackgroundTask {
    pub async fn join(mut self) -> Result<BackgroundTaskOutcome, BackgroundTaskShutdownError> {
        {
            let _shutdown_tx = self.shutdown_tx.take();
            match self.task_join.take() {
                Some(task_join) => task_join.0.await.map_err(|error| {
                    BackgroundTaskShutdownError::Join(TokioTaskJoinError::from(error))
                }),
                None => Ok(BackgroundTaskOutcome::Completed),
            }
        }
    }
    pub async fn shutdown(
        mut self,
        timeout: RequestTimeoutDuration,
    ) -> Result<BackgroundTaskOutcome, BackgroundTaskShutdownError> {
        if let Some(shutdown_tx) = self.shutdown_tx.take() {
            let _send_result = shutdown_tx.0.send(());
        }
        let Some(mut task_join) = self.task_join.take().map(|value| value.0) else {
            return Ok(BackgroundTaskOutcome::ShutdownRequested);
        };
        match tokio::time::timeout(timeout.get(), &mut task_join).await {
            Ok(result) => result.map_err(|error| {
                BackgroundTaskShutdownError::Join(TokioTaskJoinError::from(error))
            }),
            Err(_elapsed) => {
                task_join.abort();
                match task_join.await {
                    Ok(_) | Err(_) => Err(BackgroundTaskShutdownError::Timeout),
                }
            }
        }
    }
}
impl Drop for BackgroundTask {
    fn drop(&mut self) {
        if let Some(shutdown_tx) = self.shutdown_tx.take() {
            let _send_result = shutdown_tx.0.send(());
        }
        if let Some(task_join) = self.task_join.take() {
            task_join.0.abort();
        }
    }
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq)]
pub struct RunIntervalDuration(std::time::Duration);
impl TryFrom<std::time::Duration> for RunIntervalDuration {
    type Error = StdRunIntervalTryFromDurationError;
    fn try_from(value: std::time::Duration) -> Result<Self, Self::Error> {
        if value.is_zero() {
            Err(StdRunIntervalTryFromDurationError)
        } else {
            Ok(Self(value))
        }
    }
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error,
)]
#[error("{}", constants_str::RUN_INTERVAL_MUST_BE_GREATER_THAN_ZERO)]
pub struct StdRunIntervalTryFromDurationError;
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq)]
pub struct RequestTimeoutDuration(std::time::Duration);
impl RequestTimeoutDuration {
    pub(crate) const fn get(self) -> std::time::Duration {
        self.0
    }
}
impl TryFrom<std::time::Duration> for RequestTimeoutDuration {
    type Error = StdRequestTimeoutTryFromDurationError;
    fn try_from(value: std::time::Duration) -> Result<Self, Self::Error> {
        if value.is_zero() {
            Err(StdRequestTimeoutTryFromDurationError)
        } else {
            Ok(Self(value))
        }
    }
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error,
)]
#[error("{}", constants_str::REQUEST_TIMEOUT_MUST_BE_GREATER_THAN_ZERO)]
pub struct StdRequestTimeoutTryFromDurationError;
pub async fn abort_and_wait_task(task: TokioAbortTask) -> Result<(), TokioTaskJoinError> {
    task.0.abort();
    task.0.await.map_err(TokioTaskJoinError)
}
#[must_use]
#[allow(clippy::integer_division_remainder_used)]
pub fn spawn_interval_task<Run, RunFuture>(
    optional_interval: Option<RunIntervalDuration>,
    mut run: Run,
) -> Option<BackgroundTask>
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
                    return BackgroundTaskOutcome::ShutdownRequested;
                }
                _tick = timer.tick() => run().await,
            }
        }
    });
    Some(BackgroundTask {
        task_join: Some(TokioBackgroundTaskJoin::from(task_join)),
        shutdown_tx: Some(TokioBackgroundTaskShutdownSender::from(shutdown_tx)),
    })
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn aborted_task_is_awaited_and_reports_cancellation() {
        let task_join = tokio::spawn(std::future::pending::<()>());
        let result = super::abort_and_wait_task(super::TokioAbortTask::from(task_join)).await;
        assert!(result.is_err());
    }
}
