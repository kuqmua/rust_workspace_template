#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ServiceTracingFormat {
    Json,
    Text,
}

#[derive(Debug, newtype::Display, newtype::Error, newtype::FromInner)]
pub struct TracingSubscriberInitError(tracing_subscriber::util::TryInitError);

#[derive(Debug, newtype::FromInner, newtype::IntoInnerFrom)]
pub struct TokioServiceRuntime(tokio::runtime::Runtime);

#[derive(Debug, newtype::Display, newtype::Error, newtype::FromInner)]
pub struct StdServiceRuntimeIoError(std::io::Error);

pub fn build_service_runtime() -> Result<TokioServiceRuntime, StdServiceRuntimeIoError> {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .map(TokioServiceRuntime)
        .map_err(StdServiceRuntimeIoError)
}

pub fn initialize_service_tracing(
    format: ServiceTracingFormat,
) -> Result<(), TracingSubscriberInitError> {
    let filter = tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_error| {
        tracing_subscriber::EnvFilter::new(str_constants::CONFIG_TRACING_INFO)
    });
    match format {
        ServiceTracingFormat::Json => tracing_subscriber::util::SubscriberInitExt::try_init(
            tracing_subscriber::layer::SubscriberExt::with(
                tracing_subscriber::layer::SubscriberExt::with(
                    tracing_subscriber::registry(),
                    filter,
                ),
                tracing_subscriber::fmt::layer().json(),
            ),
        ),
        ServiceTracingFormat::Text => tracing_subscriber::util::SubscriberInitExt::try_init(
            tracing_subscriber::layer::SubscriberExt::with(
                tracing_subscriber::layer::SubscriberExt::with(
                    tracing_subscriber::registry(),
                    filter,
                ),
                tracing_subscriber::fmt::layer(),
            ),
        ),
    }
    .map_err(TracingSubscriberInitError)
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
