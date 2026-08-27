#[path = "lifecycle/abort_and_wait_task.rs"]
mod abort_and_wait_task;
#[path = "lifecycle/background_task.rs"]
mod background_task;
#[path = "lifecycle/background_task_outcome.rs"]
mod background_task_outcome;
#[path = "lifecycle/background_task_shutdown_error.rs"]
mod background_task_shutdown_error;
#[path = "lifecycle/request_timeout_duration.rs"]
mod request_timeout_duration;
#[path = "lifecycle/run_interval_duration.rs"]
mod run_interval_duration;
#[path = "lifecycle/spawn_interval_task.rs"]
mod spawn_interval_task;
#[path = "lifecycle/std_request_timeout_try_from_duration_error.rs"]
mod std_request_timeout_try_from_duration_error;
#[path = "lifecycle/std_run_interval_try_from_duration_error.rs"]
mod std_run_interval_try_from_duration_error;
#[path = "lifecycle/tokio_abort_task.rs"]
mod tokio_abort_task;
#[path = "lifecycle/tokio_background_task_join.rs"]
mod tokio_background_task_join;
#[path = "lifecycle/tokio_background_task_shutdown_sender.rs"]
mod tokio_background_task_shutdown_sender;
#[path = "lifecycle/tokio_task_join_error.rs"]
mod tokio_task_join_error;

pub use abort_and_wait_task::abort_and_wait_task;
pub use background_task::BackgroundTask;
pub use background_task_outcome::BackgroundTaskOutcome;
pub use background_task_shutdown_error::BackgroundTaskShutdownError;
pub use request_timeout_duration::RequestTimeoutDuration;
pub use run_interval_duration::RunIntervalDuration;
pub use spawn_interval_task::spawn_interval_task;
pub use std_request_timeout_try_from_duration_error::StdRequestTimeoutTryFromDurationError;
pub use std_run_interval_try_from_duration_error::StdRunIntervalTryFromDurationError;
pub use tokio_abort_task::TokioAbortTask;
use tokio_background_task_join::TokioBackgroundTaskJoin;
use tokio_background_task_shutdown_sender::TokioBackgroundTaskShutdownSender;
pub use tokio_task_join_error::TokioTaskJoinError;

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn aborted_task_is_awaited_and_reports_cancellation() {
        let task_join = tokio::spawn(std::future::pending::<()>());
        let result = super::abort_and_wait_task(super::TokioAbortTask::from(task_join)).await;
        assert!(result.is_err());
    }
}
