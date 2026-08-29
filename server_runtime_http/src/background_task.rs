#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug)]
#[must_use]
pub struct BackgroundTask {
    pub(super) shutdown_tx:
        Option<crate::tokio_background_task_shutdown_sender::TokioBackgroundTaskShutdownSender>,
    pub(super) task_join: Option<crate::tokio_background_task_join::TokioBackgroundTaskJoin>,
}

impl BackgroundTask {
    pub async fn join(
        mut self,
    ) -> Result<
        crate::background_task_outcome::BackgroundTaskOutcome,
        crate::background_task_shutdown_error::BackgroundTaskShutdownError,
    > {
        let _shutdown_tx = self.shutdown_tx.take();
        match self.task_join.take() {
            Some(task_join) => task_join.0.await.map_err(|error| {
                crate::background_task_shutdown_error::BackgroundTaskShutdownError::Join(
                    crate::tokio_task_join_error::TokioTaskJoinError::from(error),
                )
            }),
            None => Ok(crate::background_task_outcome::BackgroundTaskOutcome::Completed),
        }
    }

    pub async fn shutdown(
        mut self,
        timeout: crate::request_timeout_duration::RequestTimeoutDuration,
    ) -> Result<
        crate::background_task_outcome::BackgroundTaskOutcome,
        crate::background_task_shutdown_error::BackgroundTaskShutdownError,
    > {
        if let Some(shutdown_tx) = self.shutdown_tx.take() {
            let _send_result = shutdown_tx.0.send(());
        }
        let Some(mut task_join) = self.task_join.take().map(|value| value.0) else {
            return Ok(crate::background_task_outcome::BackgroundTaskOutcome::ShutdownRequested);
        };
        match tokio::time::timeout(timeout.get(), &mut task_join).await {
            Ok(result) => result.map_err(|error| {
                crate::background_task_shutdown_error::BackgroundTaskShutdownError::Join(
                    crate::tokio_task_join_error::TokioTaskJoinError::from(error),
                )
            }),
            Err(_elapsed) => {
                task_join.abort();
                match task_join.await {
                    Ok(_) | Err(_) => Err(
                        crate::background_task_shutdown_error::BackgroundTaskShutdownError::Timeout,
                    ),
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
