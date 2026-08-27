#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug)]
#[must_use]
pub struct BackgroundTask {
    pub(super) shutdown_tx: Option<super::TokioBackgroundTaskShutdownSender>,
    pub(super) task_join: Option<super::TokioBackgroundTaskJoin>,
}

impl BackgroundTask {
    pub async fn join(
        mut self,
    ) -> Result<super::BackgroundTaskOutcome, super::BackgroundTaskShutdownError> {
        let _shutdown_tx = self.shutdown_tx.take();
        match self.task_join.take() {
            Some(task_join) => task_join.0.await.map_err(|error| {
                super::BackgroundTaskShutdownError::Join(super::TokioTaskJoinError::from(error))
            }),
            None => Ok(super::BackgroundTaskOutcome::Completed),
        }
    }

    pub async fn shutdown(
        mut self,
        timeout: super::RequestTimeoutDuration,
    ) -> Result<super::BackgroundTaskOutcome, super::BackgroundTaskShutdownError> {
        if let Some(shutdown_tx) = self.shutdown_tx.take() {
            let _send_result = shutdown_tx.0.send(());
        }
        let Some(mut task_join) = self.task_join.take().map(|value| value.0) else {
            return Ok(super::BackgroundTaskOutcome::ShutdownRequested);
        };
        match tokio::time::timeout(timeout.get(), &mut task_join).await {
            Ok(result) => result.map_err(|error| {
                super::BackgroundTaskShutdownError::Join(super::TokioTaskJoinError::from(error))
            }),
            Err(_elapsed) => {
                task_join.abort();
                match task_join.await {
                    Ok(_) | Err(_) => Err(super::BackgroundTaskShutdownError::Timeout),
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
