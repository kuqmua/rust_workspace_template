#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BackgroundTaskOutcome {
    Completed,
    ShutdownRequested,
}
#[derive(Debug)]
pub struct TokioTaskJoinError(tokio::task::JoinError);
impl std::fmt::Display for TokioTaskJoinError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
impl std::error::Error for TokioTaskJoinError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.0)
    }
}
#[derive(Debug)]
pub enum BackgroundTaskShutdownError {
    Join(TokioTaskJoinError),
    Timeout,
}
impl std::fmt::Display for BackgroundTaskShutdownError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Join(error) => write!(f, "background task failed: {error}"),
            Self::Timeout => f.write_str("background task shutdown timed out"),
        }
    }
}
impl std::error::Error for BackgroundTaskShutdownError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Join(error) => Some(error),
            Self::Timeout => None,
        }
    }
}
#[derive(Debug)]
#[must_use]
pub struct BackgroundTask {
    handle: Option<TokioBackgroundTaskJoinHandle>,
    shutdown_tx: Option<TokioBackgroundTaskShutdownSender>,
}
#[derive(Debug)]
struct TokioBackgroundTaskJoinHandle(tokio::task::JoinHandle<BackgroundTaskOutcome>);
impl From<tokio::task::JoinHandle<BackgroundTaskOutcome>> for TokioBackgroundTaskJoinHandle {
    fn from(value: tokio::task::JoinHandle<BackgroundTaskOutcome>) -> Self {
        Self(value)
    }
}
#[derive(Debug)]
struct TokioBackgroundTaskShutdownSender(tokio::sync::oneshot::Sender<()>);
impl From<tokio::sync::oneshot::Sender<()>> for TokioBackgroundTaskShutdownSender {
    fn from(value: tokio::sync::oneshot::Sender<()>) -> Self {
        Self(value)
    }
}
impl BackgroundTask {
    pub async fn join(mut self) -> Result<BackgroundTaskOutcome, BackgroundTaskShutdownError> {
        let shutdown_tx = self.shutdown_tx.take();
        let result = match self.handle.take() {
            Some(handle) => handle
                .0
                .await
                .map_err(|error| BackgroundTaskShutdownError::Join(TokioTaskJoinError(error))),
            None => Ok(BackgroundTaskOutcome::Completed),
        };
        drop(shutdown_tx);
        result
    }
    pub async fn shutdown(
        mut self,
        timeout: StdRequestTimeout,
    ) -> Result<BackgroundTaskOutcome, BackgroundTaskShutdownError> {
        if let Some(shutdown_tx) = self.shutdown_tx.take() {
            let _send_result = shutdown_tx.0.send(());
        }
        let Some(mut handle) = self.handle.take().map(|value| value.0) else {
            return Ok(BackgroundTaskOutcome::ShutdownRequested);
        };
        match tokio::time::timeout(timeout.get(), &mut handle).await {
            Ok(result) => {
                result.map_err(|error| BackgroundTaskShutdownError::Join(TokioTaskJoinError(error)))
            }
            Err(_elapsed) => {
                handle.abort();
                match handle.await {
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
        if let Some(handle) = self.handle.take() {
            handle.0.abort();
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StdRunInterval(std::time::Duration);
impl TryFrom<std::time::Duration> for StdRunInterval {
    type Error = StdRunIntervalTryFromDurationError;
    fn try_from(value: std::time::Duration) -> Result<Self, Self::Error> {
        if value.is_zero() {
            Err(StdRunIntervalTryFromDurationError)
        } else {
            Ok(Self(value))
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StdRunIntervalTryFromDurationError;
impl std::fmt::Display for StdRunIntervalTryFromDurationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("run interval must be greater than zero")
    }
}
impl std::error::Error for StdRunIntervalTryFromDurationError {}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StdRequestTimeout(std::time::Duration);
impl StdRequestTimeout {
    pub(crate) const fn get(self) -> std::time::Duration {
        self.0
    }
}
impl TryFrom<std::time::Duration> for StdRequestTimeout {
    type Error = StdRequestTimeoutTryFromDurationError;
    fn try_from(value: std::time::Duration) -> Result<Self, Self::Error> {
        if value.is_zero() {
            Err(StdRequestTimeoutTryFromDurationError)
        } else {
            Ok(Self(value))
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StdRequestTimeoutTryFromDurationError;
impl std::fmt::Display for StdRequestTimeoutTryFromDurationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("request timeout must be greater than zero")
    }
}
impl std::error::Error for StdRequestTimeoutTryFromDurationError {}
#[must_use]
#[allow(clippy::integer_division_remainder_used)]
pub fn spawn_interval_task<Run, RunFuture>(
    optional_interval: Option<StdRunInterval>,
    mut run: Run,
) -> Option<BackgroundTask>
where
    Run: FnMut() -> RunFuture + Send + 'static,
    RunFuture: Future<Output = ()> + Send + 'static,
{
    let interval = optional_interval?;
    let (shutdown_tx, mut shutdown_rx) = tokio::sync::oneshot::channel();
    let handle = tokio::spawn(async move {
        let mut timer = tokio::time::interval(interval.0);
        timer.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);
        loop {
            tokio::select! {
                shutdown_result = &mut shutdown_rx => {
                    drop(shutdown_result);
                    return BackgroundTaskOutcome::ShutdownRequested;
                }
                _tick = timer.tick() => run().await,
            }
        }
    });
    Some(BackgroundTask {
        handle: Some(TokioBackgroundTaskJoinHandle::from(handle)),
        shutdown_tx: Some(TokioBackgroundTaskShutdownSender::from(shutdown_tx)),
    })
}
