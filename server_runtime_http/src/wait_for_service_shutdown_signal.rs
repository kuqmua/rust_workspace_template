// The owner module retains lint-sensitive semantics from the original implementation.
#[allow(clippy::integer_division_remainder_used)]
pub async fn wait_for_service_shutdown_signal()
-> Result<(), crate::service_runtime_io_error::ServiceRuntimeIoError> {
    #[cfg(unix)]
    {
        let mut terminate =
            tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
                .map_err(crate::service_runtime_io_error::ServiceRuntimeIoError::from)?;
        tokio::select! {
            ctrl_c = tokio::signal::ctrl_c() => ctrl_c.map_err(crate::service_runtime_io_error::ServiceRuntimeIoError::from),
            _signal = terminate.recv() => Ok(()),
        }
    }
    #[cfg(not(unix))]
    {
        tokio::signal::ctrl_c()
            .await
            .map_err(crate::domain_types::ServiceRuntimeIoError::from)
    }
}
