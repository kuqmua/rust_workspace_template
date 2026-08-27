// The owner module retains lint-sensitive semantics from the original implementation.
#[allow(clippy::integer_division_remainder_used)]
pub async fn wait_for_service_shutdown_signal() -> Result<(), super::super::ServiceRuntimeIoError> {
    let mut terminate = tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
        .map_err(super::super::ServiceRuntimeIoError::from)?;
    tokio::select! {
        ctrl_c = tokio::signal::ctrl_c() => ctrl_c.map_err(super::super::ServiceRuntimeIoError::from),
        _signal = terminate.recv() => Ok(()),
    }
}
