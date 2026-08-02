#[derive(optml::Optml, Clone, Copy, Debug, newtype::Display, newtype::FromInner)]
pub struct ServiceName(&'static str);

#[derive(optml::Optml, Debug, thiserror::Error, newtype::FromInner)]
#[error(transparent)]
pub struct OpentelemetryOtlpExporterBuildError(opentelemetry_otlp::ExporterBuildError);

#[derive(optml::Optml, Debug, newtype::FromInner)]
struct OpentelemetrySdkTracerProvider(opentelemetry_sdk::trace::SdkTracerProvider);

#[derive(optml::Optml, Debug, thiserror::Error, newtype::FromInner)]
#[error(transparent)]
pub struct TracingSubscriberInitError(tracing_subscriber::util::TryInitError);

#[derive(optml::Optml, Debug, thiserror::Error)]
pub enum ObservabilityInitError {
    #[error("failed to build OTLP span exporter: {0}")]
    Exporter(OpentelemetryOtlpExporterBuildError),
    #[error("failed to install tracing subscriber: {0}")]
    Subscriber(TracingSubscriberInitError),
}

#[derive(optml::Optml, Debug, thiserror::Error, newtype::FromInner)]
#[error("failed to shut down OpenTelemetry tracer provider: {0}")]
pub struct OpentelemetrySdkObservabilityShutdownError(opentelemetry_sdk::error::OTelSdkError);

#[derive(optml::Optml, Debug)]
pub struct ObservabilityGuard {
    tracer_provider: Option<OpentelemetrySdkTracerProvider>,
}

impl ObservabilityGuard {
    pub fn shutdown(mut self) -> Result<(), OpentelemetrySdkObservabilityShutdownError> {
        let Some(tracer_provider) = self.tracer_provider.take() else {
            return Ok(());
        };
        tracer_provider
            .0
            .shutdown()
            .map_err(OpentelemetrySdkObservabilityShutdownError)
    }
}

impl Drop for ObservabilityGuard {
    fn drop(&mut self) {
        let Some(tracer_provider) = self.tracer_provider.take() else {
            return;
        };
        if let Err(error) = tracer_provider.0.shutdown() {
            tracing::error!(error = %error, "OpenTelemetry tracer provider shutdown failed");
        }
    }
}

pub fn initialize_service_observability(
    format: crate::ServiceTracingFormat,
    service_name: ServiceName,
) -> Result<ObservabilityGuard, ObservabilityInitError> {
    opentelemetry::global::set_text_map_propagator(
        opentelemetry_sdk::propagation::TraceContextPropagator::new(),
    );
    let exporter = opentelemetry_otlp::SpanExporter::builder()
        .with_http()
        .build()
        .map_err(|error| {
            ObservabilityInitError::Exporter(OpentelemetryOtlpExporterBuildError::from(error))
        })?;
    let tracer_provider = opentelemetry_sdk::trace::SdkTracerProvider::builder()
        .with_batch_exporter(exporter)
        .with_resource(
            opentelemetry_sdk::Resource::builder()
                .with_service_name(service_name.0)
                .build(),
        )
        .build();
    let tracer = opentelemetry::trace::TracerProvider::tracer(&tracer_provider, service_name.0);
    opentelemetry::global::set_tracer_provider(tracer_provider.clone());
    let filter = tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_error| {
        tracing_subscriber::EnvFilter::new(str_constants::CONFIG_TRACING_INFO)
    });
    let init_result = match format {
        crate::ServiceTracingFormat::Json => tracing_subscriber::util::SubscriberInitExt::try_init(
            tracing_subscriber::layer::SubscriberExt::with(
                tracing_subscriber::layer::SubscriberExt::with(
                    tracing_subscriber::layer::SubscriberExt::with(
                        tracing_subscriber::registry(),
                        filter,
                    ),
                    tracing_opentelemetry::layer().with_tracer(tracer),
                ),
                tracing_subscriber::fmt::layer().json(),
            ),
        ),
        crate::ServiceTracingFormat::Text => tracing_subscriber::util::SubscriberInitExt::try_init(
            tracing_subscriber::layer::SubscriberExt::with(
                tracing_subscriber::layer::SubscriberExt::with(
                    tracing_subscriber::layer::SubscriberExt::with(
                        tracing_subscriber::registry(),
                        filter,
                    ),
                    tracing_opentelemetry::layer().with_tracer(tracer),
                ),
                tracing_subscriber::fmt::layer(),
            ),
        ),
    };
    if let Err(error) = init_result {
        if let Err(shutdown_error) = tracer_provider.shutdown() {
            tracing::error!(
                error = %shutdown_error,
                "OpenTelemetry cleanup after subscriber initialization failure failed"
            );
        }
        return Err(ObservabilityInitError::Subscriber(
            TracingSubscriberInitError::from(error),
        ));
    }
    Ok(ObservabilityGuard {
        tracer_provider: Some(OpentelemetrySdkTracerProvider::from(tracer_provider)),
    })
}

#[cfg(test)]
mod tests {
    #[test]
    fn empty_guard_shutdown_is_idempotent_and_service_name_displays() {
        let guard = super::ObservabilityGuard {
            tracer_provider: None,
        };
        guard.shutdown().expect("599ca192");
        assert_eq!(
            super::ServiceName::from("notification_service").to_string(),
            "notification_service"
        );
    }
    #[test]
    fn guard_shuts_down_owned_tracer_provider() {
        let exporter = opentelemetry_sdk::trace::InMemorySpanExporterBuilder::new().build();
        let tracer_provider = opentelemetry_sdk::trace::SdkTracerProvider::builder()
            .with_simple_exporter(exporter.clone())
            .build();
        let guard = super::ObservabilityGuard {
            tracer_provider: Some(super::OpentelemetrySdkTracerProvider::from(tracer_provider)),
        };
        guard.shutdown().expect("8d66ae8c");
        assert!(exporter.is_shutdown_called());
    }
    #[test]
    fn dropping_guard_shuts_down_owned_tracer_provider() {
        let exporter = opentelemetry_sdk::trace::InMemorySpanExporterBuilder::new().build();
        let tracer_provider = opentelemetry_sdk::trace::SdkTracerProvider::builder()
            .with_simple_exporter(exporter.clone())
            .build();
        let guard = super::ObservabilityGuard {
            tracer_provider: Some(super::OpentelemetrySdkTracerProvider::from(tracer_provider)),
        };
        drop(guard);
        assert!(exporter.is_shutdown_called());
    }
}
