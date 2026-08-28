pub use crate::abort_and_wait_task::abort_and_wait_task;
pub use crate::background_task::BackgroundTask;
pub use crate::background_task_outcome::BackgroundTaskOutcome;
pub use crate::background_task_shutdown_error::BackgroundTaskShutdownError;
pub use crate::request_timeout_duration::RequestTimeoutDuration;
pub use crate::run_interval_duration::RunIntervalDuration;
pub use crate::spawn_interval_task::spawn_interval_task;
pub use crate::std_request_timeout_try_from_duration_error::StdRequestTimeoutTryFromDurationError;
pub use crate::std_run_interval_try_from_duration_error::StdRunIntervalTryFromDurationError;
pub use crate::tokio_abort_task::TokioAbortTask;
use crate::tokio_background_task_join::TokioBackgroundTaskJoin;
use crate::tokio_background_task_shutdown_sender::TokioBackgroundTaskShutdownSender;
pub use crate::tokio_task_join_error::TokioTaskJoinError;

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn aborted_task_is_awaited_and_reports_cancellation() {
        let task_join = tokio::spawn(std::future::pending::<()>());
        let result = super::abort_and_wait_task(super::TokioAbortTask::from(task_join)).await;
        assert!(result.is_err());
    }
}

// Root-owned module compatibility wrappers.
mod abort_and_wait_task {
    pub use crate::abort_and_wait_task::*;
}
mod background_task {
    pub use crate::background_task::*;
}
mod background_task_outcome {
    pub use crate::background_task_outcome::*;
}
mod background_task_shutdown_error {
    pub use crate::background_task_shutdown_error::*;
}
mod request_timeout_duration {
    pub use crate::request_timeout_duration::*;
}
mod run_interval_duration {
    pub use crate::run_interval_duration::*;
}
mod spawn_interval_task {
    pub use crate::spawn_interval_task::*;
}
mod std_request_timeout_try_from_duration_error {
    pub use crate::std_request_timeout_try_from_duration_error::*;
}
mod std_run_interval_try_from_duration_error {
    pub use crate::std_run_interval_try_from_duration_error::*;
}
mod tokio_abort_task {
    pub use crate::tokio_abort_task::*;
}
mod tokio_background_task_join {
    pub use crate::tokio_background_task_join::*;
}
mod tokio_background_task_shutdown_sender {
    pub use crate::tokio_background_task_shutdown_sender::*;
}
mod tokio_task_join_error {
    pub use crate::tokio_task_join_error::*;
}
