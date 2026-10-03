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
        match self.task_join.as_mut() {
            Some(task_join) => task_join.join().await,
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
        let Some(task_join) = self.task_join.as_mut() else {
            return Ok(crate::background_task_outcome::BackgroundTaskOutcome::ShutdownRequested);
        };
        match tokio::time::timeout(request_timeout_duration.get(), task_join.join()).await {
            Ok(result) => result,
            Err(_elapsed) => {
                task_join.abort();
                match task_join.join().await {
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

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn test_background_task_without_join_handle_preserves_shutdown_signal_semantics() {
        let (join_sender, join_receiver) = tokio::sync::oneshot::channel::<()>();
        let join_result = crate::background_task::BackgroundTask::new(
            Some(crate::tokio_background_task_shutdown_sender::TokioBackgroundTaskShutdownSender::from(join_sender)),
            None,
        ).join().await;
        assert!(matches!(
            join_result,
            Ok(crate::background_task_outcome::BackgroundTaskOutcome::Completed)
        ));
        assert!(
            join_receiver
                .await
                .is_err_and(|error| !error.to_string().is_empty())
        );
        let shutdown_matches = match crate::request_timeout_duration::RequestTimeoutDuration::try_from(std::time::Duration::from_secs(1u64)) {
            Ok(timeout) => {
                let (shutdown_sender, shutdown_receiver) = tokio::sync::oneshot::channel::<()>();
                let result = crate::background_task::BackgroundTask::new(
                    Some(crate::tokio_background_task_shutdown_sender::TokioBackgroundTaskShutdownSender::from(shutdown_sender)),
                    None,
                ).shutdown(timeout).await;
                matches!(result, Ok(crate::background_task_outcome::BackgroundTaskOutcome::ShutdownRequested))
                    && matches!(shutdown_receiver.await, Ok(()))
            }
            Err(crate::std_request_timeout_try_from_duration_error::StdRequestTimeoutTryFromDurationError::Zero) => false,
        };
        assert!(shutdown_matches);
    }
    #[tokio::test(start_paused = true)]
    async fn test_background_task_drop_signals_shutdown_and_aborts_started_task() {
        let (shutdown_sender, shutdown_receiver) = tokio::sync::oneshot::channel::<()>();
        let (started_sender, started_receiver) = tokio::sync::oneshot::channel::<()>();
        let (retained_sender, retained_receiver) = tokio::sync::oneshot::channel::<()>();
        let task_join = tokio::spawn(async move {
            let completion_sender = retained_sender;
            let _started_signal = started_sender.send(());
            let outcome =
                std::future::pending::<crate::background_task_outcome::BackgroundTaskOutcome>()
                    .await;
            drop(completion_sender);
            outcome
        });
        let background_task = crate::background_task::BackgroundTask::new(
            Some(crate::tokio_background_task_shutdown_sender::TokioBackgroundTaskShutdownSender::from(shutdown_sender)),
            Some(crate::tokio_background_task_join::TokioBackgroundTaskJoin::from(task_join)),
        );
        assert_eq!(started_receiver.await, Ok(()));
        drop(background_task);
        assert_eq!(shutdown_receiver.await, Ok(()));
        assert!(matches!(
            tokio::time::timeout(std::time::Duration::from_secs(1u64), retained_receiver).await,
            Ok(Err(_))
        ));
    }
}
