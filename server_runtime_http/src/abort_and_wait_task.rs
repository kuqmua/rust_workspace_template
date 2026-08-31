pub async fn abort_and_wait_task(
    task: crate::tokio_abort_task::TokioAbortTask,
) -> Result<(), crate::tokio_task_join_error::TokioTaskJoinError> {
    let task_join = task.into_inner();
    task_join.abort();
    task_join
        .await
        .map_err(crate::tokio_task_join_error::TokioTaskJoinError::from)
}
