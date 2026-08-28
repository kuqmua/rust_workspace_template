pub fn init_service_observability(
    format: crate::service_tracing_format::ServiceTracingFormat,
    service_name: crate::initialization::ServiceName,
) -> Result<crate::initialization::ObservabilityGuard, crate::initialization::ObservabilityInitError>
{
    opentelemetry::global::set_text_map_propagator(
        opentelemetry_sdk::propagation::TraceContextPropagator::new(),
    );
    let exporter = opentelemetry_otlp::SpanExporter::builder()
        .with_http()
        .build()
        .map_err(|error| {
            crate::initialization::ObservabilityInitError::Exporter(
                crate::initialization::OpentelemetryOtlpExporterBuildError::from(error),
            )
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
        tracing_subscriber::EnvFilter::new(constants_str::CONFIG_TRACING_INFO)
    });
    let init_result = match format {
        crate::service_tracing_format::ServiceTracingFormat::Json => {
            tracing_subscriber::util::SubscriberInitExt::try_init(
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
            )
        }
        crate::service_tracing_format::ServiceTracingFormat::Text => {
            tracing_subscriber::util::SubscriberInitExt::try_init(
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
            )
        }
    };
    if let Err(error) = init_result {
        if let Err(shutdown_error) = tracer_provider.shutdown() {
            tracing::error!(
                error = %shutdown_error,
                "OpenTelemetry cleanup after subscriber initialization failure failed"
            );
        }
        return Err(crate::initialization::ObservabilityInitError::Subscriber(
            crate::initialization::TracingSubscriberInitError::from(error),
        ));
    }
    Ok(crate::initialization::ObservabilityGuard {
        tracer_provider: Some(crate::initialization::OpentelemetrySdkTracerProvider::from(
            tracer_provider,
        )),
    })
}
