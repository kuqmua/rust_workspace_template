pub async fn wait_for_service_shutdown_signal() -> Result<(), super::super::ServiceRuntimeIoError> {
    tokio::signal::ctrl_c()
        .await
        .map_err(super::super::ServiceRuntimeIoError::from)
}
