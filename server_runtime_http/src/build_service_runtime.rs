pub fn build_service_runtime() -> Result<super::TokioServiceRuntime, super::ServiceRuntimeIoError> {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .map(super::TokioServiceRuntime::from)
        .map_err(super::ServiceRuntimeIoError::from)
}
