#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Debug, proc_macro_new::New)]
#[constructor(pub(crate))]
#[must_use]
pub struct BackgroundTask {
    shutdown_tx:
        Option<crate::tokio_background_task_shutdown_sender::TokioBackgroundTaskShutdownSender>,
    task_join: Option<crate::tokio_background_task_join::TokioBackgroundTaskJoin>,
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
            Some(task_join) => tokio::task::JoinHandle::from(task_join)
                .await
                .map_err(|error| {
                    crate::background_task_shutdown_error::BackgroundTaskShutdownError::Join(
                        crate::tokio_task_join_error::TokioTaskJoinError::from(error),
                    )
                }),
            None => Ok(crate::background_task_outcome::BackgroundTaskOutcome::Completed),
        }
    }

    pub async fn shutdown(
        mut self,
        request_timeout_duration: crate::request_timeout_duration::RequestTimeoutDuration,
    ) -> Result<
        crate::background_task_outcome::BackgroundTaskOutcome,
        crate::background_task_shutdown_error::BackgroundTaskShutdownError,
    > {
        if let Some(shutdown_tx) = self.shutdown_tx.take() {
            let _send_result = tokio::sync::oneshot::Sender::from(shutdown_tx).send(());
        }
        let Some(mut task_join) = self.task_join.take().map(tokio::task::JoinHandle::from) else {
            return Ok(crate::background_task_outcome::BackgroundTaskOutcome::ShutdownRequested);
        };
        match tokio::time::timeout(request_timeout_duration.get(), &mut task_join).await {
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
        let _send_result = self
            .shutdown_tx
            .take()
            .map(|shutdown_tx| tokio::sync::oneshot::Sender::from(shutdown_tx).send(()));
        let _abort_result = self.task_join.take().map(|task_join| task_join.abort());
    }
}
