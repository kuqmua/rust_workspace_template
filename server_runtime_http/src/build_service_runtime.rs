pub fn build_service_runtime() -> Result<
    crate::tokio_service_runtime::TokioServiceRuntime,
    crate::service_runtime_io_error::ServiceRuntimeIoError,
> {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .map(crate::tokio_service_runtime::TokioServiceRuntime::from)
        .map_err(crate::service_runtime_io_error::ServiceRuntimeIoError::from)
}
