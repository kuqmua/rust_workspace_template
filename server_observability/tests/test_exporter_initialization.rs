#![allow(
    unused_crate_dependencies,
    reason = "this isolated integration target exercises exporter initialization through the public observability API; other dependencies support library implementation"
)]

#[cfg(test)]
mod tests {
    #[test]
    #[ignore = "requires an isolated process with an OTLP endpoint configured and OTEL_EXPORTER_OTLP_TRACES_COMPRESSION=unsupported"]
    fn test_invalid_exporter_compression_preserves_error_and_leaves_subscriber_uninstalled() {
        assert!(
            [
                opentelemetry_otlp::OTEL_EXPORTER_OTLP_ENDPOINT,
                opentelemetry_otlp::OTEL_EXPORTER_OTLP_TRACES_ENDPOINT,
            ]
            .iter()
            .any(|name| std::env::var_os(name).is_some_and(|value| !value.is_empty()))
        );
        assert!(
            std::env::var(opentelemetry_otlp::OTEL_EXPORTER_OTLP_TRACES_COMPRESSION)
                .is_ok_and(|value| value == stringify!(unsupported))
        );
        assert!(opentelemetry_otlp::SpanExporter::builder()
            .with_http()
            .build()
            .is_err_and(|exporter_build_error| {
                assert!(matches!(
                    &exporter_build_error,
                    opentelemetry_otlp::ExporterBuildError::UnsupportedCompressionAlgorithm(value)
                        if value == stringify!(unsupported)
                ));
                let expected_diagnostic = exporter_build_error.to_string();
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
                            server_observability::observability_init_error::ObservabilityInitError::Exporter(opentelemetry_otlp_exporter_build_error) => {
                                opentelemetry_otlp_exporter_build_error.to_string() == expected_diagnostic
                                    && diagnostic.ends_with(&expected_diagnostic)
                            }
                            server_observability::observability_init_error::ObservabilityInitError::Subscriber(_) => false,
                        }
                    })
                })
            }));
        tracing_subscriber::util::SubscriberInitExt::try_init(tracing_subscriber::registry())
            .expect(constants_str::DIAGNOSTIC_A5A6E8A3);
    }
}
