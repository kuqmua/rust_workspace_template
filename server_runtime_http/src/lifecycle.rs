#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn aborted_task_is_awaited_and_reports_cancellation() {
        let task_join = tokio::spawn(std::future::pending::<()>());
        let result = crate::abort_and_wait_task::abort_and_wait_task(
            crate::tokio_abort_task::TokioAbortTask::from(task_join),
        )
        .await;
        assert!(result.is_err());
    }
}

// Root-owned module compatibility wrappers.
mod abort_and_wait_task {}
mod background_task {}
mod background_task_outcome {}
mod background_task_shutdown_error {}
mod request_timeout_duration {}
mod run_interval_duration {}
mod spawn_interval_task {}
mod std_request_timeout_try_from_duration_error {}
mod std_run_interval_try_from_duration_error {}
mod tokio_abort_task {}
mod tokio_background_task_join {}
mod tokio_background_task_shutdown_sender {}
mod tokio_task_join_error {}
