pub async fn acquire_permit(
    semaphore: super::ArcTokioSemaphore,
    wait_timeout: super::PermitWaitTimeoutDuration,
    retry_after: super::RetryAfterSecs,
) -> Result<super::TokioOwnedSemaphorePermit, super::AcquirePermitError> {
    match tokio::time::timeout(wait_timeout.0, semaphore.0.acquire_owned()).await {
        Ok(Ok(permit)) => Ok(super::TokioOwnedSemaphorePermit::from(permit)),
        Ok(Err(error)) => Err(super::AcquirePermitError::Closed(
            super::TokioAcquireError::from(error),
        )),
        Err(_elapsed) => Err(super::AcquirePermitError::Timeout(retry_after)),
    }
}
