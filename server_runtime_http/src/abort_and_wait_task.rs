pub async fn abort_and_wait_task(
    task: crate::tokio_abort_task::TokioAbortTask,
) -> Result<(), crate::tokio_task_join_error::TokioTaskJoinError> {
    task.0.abort();
    task.0
        .await
        .map_err(crate::tokio_task_join_error::TokioTaskJoinError::from)
}
