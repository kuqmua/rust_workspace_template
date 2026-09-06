pub fn init_service_observability(
    service_tracing_format: crate::service_tracing_format::ServiceTracingFormat,
    service_name: crate::service_name::ServiceName,
) -> Result<
    crate::observability_guard::ObservabilityGuard,
    crate::observability_init_error::ObservabilityInitError,
> {
    opentelemetry::global::set_text_map_propagator(
        opentelemetry_sdk::propagation::TraceContextPropagator::new(),
    );
    let otlp_export_mode = crate::otlp_export_mode::OtlpExportMode::from(
        [
            opentelemetry_otlp::OTEL_EXPORTER_OTLP_ENDPOINT,
            opentelemetry_otlp::OTEL_EXPORTER_OTLP_TRACES_ENDPOINT,
        ]
        .iter()
        .any(|name| std::env::var_os(name).is_some_and(|value| !value.is_empty())),
    );
    let tracer_provider = crate::initialize_otlp_tracer_provider::initialize_otlp_tracer_provider(
        otlp_export_mode,
        service_name,
    )?;
    let telemetry_layer = tracer_provider
        .as_ref()
        .map(|opentelemetry_sdk_tracer_provider| {
            let tracer = opentelemetry::trace::TracerProvider::tracer(
                opentelemetry_sdk_tracer_provider.get(),
                service_name.get(),
            );
            opentelemetry::global::set_tracer_provider(
                opentelemetry_sdk_tracer_provider.get().clone(),
            );
            tracing_opentelemetry::layer().with_tracer(tracer)
        });
    let observability_guard = crate::observability_guard::ObservabilityGuard::from(tracer_provider);
    let filter = tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_error| {
        tracing_subscriber::EnvFilter::new(constants_str::CONFIG_TRACING_INFO)
    });
    let init_result = match service_tracing_format {
        crate::service_tracing_format::ServiceTracingFormat::Json => {
            tracing_subscriber::util::SubscriberInitExt::try_init(
                tracing_subscriber::layer::SubscriberExt::with(
                    tracing_subscriber::layer::SubscriberExt::with(
                        tracing_subscriber::layer::SubscriberExt::with(
                            tracing_subscriber::registry(),
                            filter,
                        ),
                        telemetry_layer,
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
                        telemetry_layer,
                    ),
                    tracing_subscriber::fmt::layer(),
                ),
            )
        }
    };
    if let Err(error) = init_result {
        if let Err(shutdown_error) = observability_guard.shutdown() {
            tracing::error!(
                error = %shutdown_error,
                message = %constants_str::TRACING_OTEL_CLEANUP_FAILED,
            );
        }
        return Err(
            crate::observability_init_error::ObservabilityInitError::Subscriber(
                crate::tracing_subscriber_init_error::TracingSubscriberInitError::from(error),
            ),
        );
    }
    Ok(observability_guard)
}
