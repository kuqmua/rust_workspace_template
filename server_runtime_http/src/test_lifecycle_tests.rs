#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn test_aborted_task_is_awaited_and_reports_cancellation() {
        let task_join = tokio::spawn(std::future::pending::<()>());
        let result = crate::abort_and_wait_task::abort_and_wait_task(
            crate::tokio_abort_task::TokioAbortTask::from(task_join),
        )
        .await;
        assert!(result.is_err());
    }
}

// Root-owned module compatibility wrappers.
