#![allow(
    unused_crate_dependencies,
    reason = "this isolated integration target exercises subscriber initialization through the public observability API; other dependencies support library implementation"
)]

#[cfg(test)]
mod tests {
    #[test]
    #[ignore = "requires an isolated process with OTLP disabled or an owned loopback endpoint and OTEL_TRACES_SAMPLER=always_on"]
    fn test_existing_subscriber_rejects_both_service_formats_with_original_diagnostic() {
        let otlp_export_enabled = [
            opentelemetry_otlp::OTEL_EXPORTER_OTLP_ENDPOINT,
            opentelemetry_otlp::OTEL_EXPORTER_OTLP_TRACES_ENDPOINT,
        ]
        .iter()
        .any(|name| std::env::var_os(name).is_some_and(|value| !value.is_empty()));
        if otlp_export_enabled {
            assert!(
                std::env::var(stringify!(OTEL_TRACES_SAMPLER))
                    .is_ok_and(|value| value == stringify!(always_on))
            );
        }
        tracing_subscriber::util::SubscriberInitExt::try_init(tracing_subscriber::registry())
            .expect(constants_str::DIAGNOSTIC_14F0D6A2);
        assert!(tracing_subscriber::util::SubscriberInitExt::try_init(
            tracing_subscriber::registry()
        )
        .is_err_and(|registration_error| {
            let expected_diagnostic = registration_error.to_string();
            [
                server_observability::service_tracing_format::ServiceTracingFormat::Json,
                server_observability::service_tracing_format::ServiceTracingFormat::Text,
            ]
            .into_iter()
            .all(|service_tracing_format| {
                server_observability::init_service_observability::init_service_observability(
                    service_tracing_format,
                    server_observability::service_name::ServiceName::from(
                        constants_str::WORKSPACE_SCAFFOLD_NOTIFICATION_SERVICE,
                    ),
                )
                .is_err_and(|observability_init_error| {
                    let diagnostic = observability_init_error.to_string();
                    match observability_init_error {
                        server_observability::observability_init_error::ObservabilityInitError::Subscriber(tracing_subscriber_init_error) => {
                            tracing_subscriber_init_error.to_string() == expected_diagnostic
                                && diagnostic.ends_with(&expected_diagnostic)
                                && (!otlp_export_enabled || !opentelemetry::trace::Span::is_recording(
                                    &opentelemetry::trace::Tracer::start(
                                        &opentelemetry::global::tracer(constants_str::WORKSPACE_SCAFFOLD_NOTIFICATION_SERVICE),
                                        constants_str::WORKSPACE_SCAFFOLD_NOTIFICATION_SERVICE,
                                    ),
                                ))
                        }
                        server_observability::observability_init_error::ObservabilityInitError::Exporter(_) => false,
                    }
                })
            })
        }));
    }
}
