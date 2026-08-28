pub async fn abort_and_wait_task(
    task: super::TokioAbortTask,
) -> Result<(), super::TokioTaskJoinError> {
    task.0.abort();
    task.0.await.map_err(super::TokioTaskJoinError::from)
}
