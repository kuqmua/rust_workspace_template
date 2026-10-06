#[cfg(test)]
mod tests {
    #[test]
    fn test_otlp_export_requires_a_configured_endpoint() {
        assert_eq!(
            crate::otlp_export_mode::OtlpExportMode::from(false),
            crate::otlp_export_mode::OtlpExportMode::Disabled,
        );
        assert_eq!(
            crate::otlp_export_mode::OtlpExportMode::from(true),
            crate::otlp_export_mode::OtlpExportMode::Enabled,
        );
    }

    #[test]
    fn test_disabled_otlp_does_not_create_a_tracer_provider() {
        let tracer_provider =
            crate::initialize_otlp_tracer_provider::initialize_otlp_tracer_provider(
                crate::otlp_export_mode::OtlpExportMode::Disabled,
                crate::service_name::ServiceName::from(
                    constants_str::WORKSPACE_SCAFFOLD_NOTIFICATION_SERVICE,
                ),
            );
        assert!(tracer_provider.is_ok_and(|option| option.is_none()));
    }

    #[test]
    fn test_empty_guard_shutdown_is_idempotent_and_service_name_displays() {
        let guard = crate::observability_guard::ObservabilityGuard::from(None);
        guard.shutdown().expect(constants_str::DIAGNOSTIC_599CA192);
        assert_eq!(
            crate::service_name::ServiceName::from(
                constants_str::WORKSPACE_SCAFFOLD_NOTIFICATION_SERVICE
            )
            .to_string(),
            constants_str::WORKSPACE_SCAFFOLD_NOTIFICATION_SERVICE
        );
    }
    #[test]
    fn test_guard_shuts_down_owned_tracer_provider() {
        let exporter = opentelemetry_sdk::trace::InMemorySpanExporterBuilder::new().build();
        let tracer_provider = opentelemetry_sdk::trace::SdkTracerProvider::builder()
            .with_simple_exporter(exporter.clone())
            .build();
        let guard = crate::observability_guard::ObservabilityGuard::from(Some(
            crate::opentelemetry_sdk_tracer_provider::OpentelemetrySdkTracerProvider::from(
                tracer_provider,
            ),
        ));
        guard.shutdown().expect(constants_str::DIAGNOSTIC_8D66AE8C);
        assert!(exporter.is_shutdown_called());
    }
    #[test]
    fn test_dropping_guard_shuts_down_owned_tracer_provider() {
        let exporter = opentelemetry_sdk::trace::InMemorySpanExporterBuilder::new().build();
        let tracer_provider = opentelemetry_sdk::trace::SdkTracerProvider::builder()
            .with_simple_exporter(exporter.clone())
            .build();
        let guard = crate::observability_guard::ObservabilityGuard::from(Some(
            crate::opentelemetry_sdk_tracer_provider::OpentelemetrySdkTracerProvider::from(
                tracer_provider,
            ),
        ));
        drop(guard);
        assert!(exporter.is_shutdown_called());
    }

    #[test]
    fn test_guard_handles_already_shutdown_provider_on_shutdown_and_drop() {
        [false, true].into_iter().fold((), |(), drop_guard| {
            let exporter = opentelemetry_sdk::trace::InMemorySpanExporterBuilder::new().build();
            let tracer_provider = opentelemetry_sdk::trace::SdkTracerProvider::builder()
                .with_simple_exporter(exporter.clone())
                .build();
            tracer_provider
                .shutdown()
                .expect(constants_str::DIAGNOSTIC_67AA7433);
            let guard = crate::observability_guard::ObservabilityGuard::from(Some(
                crate::opentelemetry_sdk_tracer_provider::OpentelemetrySdkTracerProvider::from(
                    tracer_provider,
                ),
            ));
            if drop_guard {
                drop(guard);
            } else {
                let expected_error = crate::opentelemetry_sdk_observability_shutdown_error::OpentelemetrySdkObservabilityShutdownError::from(
                    opentelemetry_sdk::error::OTelSdkError::AlreadyShutdown,
                );
                assert!(guard.shutdown().is_err_and(|error| error.to_string() == expected_error.to_string()));
            }
            assert!(exporter.is_shutdown_called());
        });
    }
}
