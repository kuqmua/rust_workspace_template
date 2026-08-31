pub async fn acquire_permit(
    semaphore: crate::arc_tokio_semaphore::ArcTokioSemaphore,
    wait_timeout: crate::permit_wait_timeout_duration::PermitWaitTimeoutDuration,
    retry_after: crate::retry_after_secs::RetryAfterSecs,
) -> Result<
    crate::tokio_owned_semaphore_permit::TokioOwnedSemaphorePermit,
    crate::acquire_permit_error::AcquirePermitError,
> {
    match tokio::time::timeout(wait_timeout.get(), semaphore.into_inner().acquire_owned()).await {
        Ok(Ok(permit)) => {
            Ok(crate::tokio_owned_semaphore_permit::TokioOwnedSemaphorePermit::from(permit))
        }
        Ok(Err(error)) => Err(crate::acquire_permit_error::AcquirePermitError::Closed(
            crate::tokio_acquire_error::TokioAcquireError::from(error),
        )),
        Err(_elapsed) => Err(crate::acquire_permit_error::AcquirePermitError::Timeout(
            retry_after,
        )),
    }
}
