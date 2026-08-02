#[derive(optml::Optml, Debug, newtype::FromInner, newtype::IntoInnerFrom)]
pub struct TokioServiceRuntime(tokio::runtime::Runtime);

#[derive(optml::Optml, Debug, thiserror::Error, newtype::FromInner)]
#[error("{0}")]
pub struct StdServiceRuntimeIoError(std::io::Error);

pub fn build_service_runtime() -> Result<TokioServiceRuntime, StdServiceRuntimeIoError> {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .map(TokioServiceRuntime)
        .map_err(StdServiceRuntimeIoError)
}

#[cfg(not(unix))]
pub async fn wait_for_service_shutdown_signal() -> Result<(), StdServiceRuntimeIoError> {
    tokio::signal::ctrl_c()
        .await
        .map_err(StdServiceRuntimeIoError)
}

#[cfg(unix)]
#[allow(
    clippy::integer_division_remainder_used,
    reason = "tokio::select macro expansion uses integer remainder internally"
)]
pub async fn wait_for_service_shutdown_signal() -> Result<(), StdServiceRuntimeIoError> {
    let mut terminate = tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
        .map_err(StdServiceRuntimeIoError)?;
    tokio::select! {
        ctrl_c = tokio::signal::ctrl_c() => ctrl_c.map_err(StdServiceRuntimeIoError),
        _signal = terminate.recv() => Ok(()),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn service_runtime_builder_enables_tokio_runtime() {
        let wrapped_runtime = super::build_service_runtime().expect("5ecc3726");
        let runtime = tokio::runtime::Runtime::from(wrapped_runtime);
        assert_eq!(runtime.block_on(async { 2u8 + 2u8 }), 4u8);
    }
}
