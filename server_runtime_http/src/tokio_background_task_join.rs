#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    proc_macro_newtype_from_inner::FromInner,
)]
pub(super) struct TokioBackgroundTaskJoin(
    tokio::task::JoinHandle<crate::background_task_outcome::BackgroundTaskOutcome>,
);

impl TokioBackgroundTaskJoin {
    pub(super) async fn join(
        &mut self,
    ) -> Result<
        crate::background_task_outcome::BackgroundTaskOutcome,
        crate::background_task_shutdown_error::BackgroundTaskShutdownError,
    > {
        let background_task_outcome = (&mut self.0).await.map_err(|error| {
            crate::background_task_shutdown_error::BackgroundTaskShutdownError::Join(
                crate::tokio_task_join_error::TokioTaskJoinError::from(error),
            )
        })?;
        match background_task_outcome {
            crate::background_task_outcome::BackgroundTaskOutcome::IntervalOverflow => {
                Err(crate::background_task_shutdown_error::BackgroundTaskShutdownError::IntervalOverflow)
            }
            crate::background_task_outcome::BackgroundTaskOutcome::Completed
            | crate::background_task_outcome::BackgroundTaskOutcome::ShutdownRequested => Ok(background_task_outcome),
        }
    }

    pub(super) fn abort(&self) {
        self.0.abort();
    }
}
