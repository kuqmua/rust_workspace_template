#[derive(Debug, newtype::FromInner, newtype::IntoInnerFrom)]
pub struct TokioServiceRuntime(tokio::runtime::Runtime);

#[derive(Debug, thiserror::Error, newtype::FromInner)]
#[error("{0}")]
pub struct StdServiceRuntimeIoError(std::io::Error);

pub fn build_service_runtime() -> Result<TokioServiceRuntime, StdServiceRuntimeIoError> {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .map(TokioServiceRuntime)
        .map_err(StdServiceRuntimeIoError)
}

pub async fn wait_for_service_shutdown_signal() -> Result<(), StdServiceRuntimeIoError> {
    tokio::signal::ctrl_c()
        .await
        .map_err(StdServiceRuntimeIoError)
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
