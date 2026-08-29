#[cfg(test)]
mod tests {
    #[test]
    fn empty_guard_shutdown_is_idempotent_and_service_name_displays() {
        let guard = crate::observability_guard::ObservabilityGuard {
            tracer_provider: None,
        };
        guard.shutdown().expect("599ca192 empty_guard_shutdown_is_idempotent_and_service_name_displays invariant must hold");
        assert_eq!(
            crate::service_name::ServiceName::from("notification_service").to_string(),
            "notification_service"
        );
    }
    #[test]
    fn guard_shuts_down_owned_tracer_provider() {
        let exporter = opentelemetry_sdk::trace::InMemorySpanExporterBuilder::new().build();
        let tracer_provider = opentelemetry_sdk::trace::SdkTracerProvider::builder()
            .with_simple_exporter(exporter.clone())
            .build();
        let guard = crate::observability_guard::ObservabilityGuard {
            tracer_provider: Some(
                crate::opentelemetry_sdk_tracer_provider::OpentelemetrySdkTracerProvider::from(
                    tracer_provider,
                ),
            ),
        };
        guard
            .shutdown()
            .expect("8d66ae8c guard_shuts_down_owned_tracer_provider invariant must hold");
        assert!(exporter.is_shutdown_called());
    }
    #[test]
    fn dropping_guard_shuts_down_owned_tracer_provider() {
        let exporter = opentelemetry_sdk::trace::InMemorySpanExporterBuilder::new().build();
        let tracer_provider = opentelemetry_sdk::trace::SdkTracerProvider::builder()
            .with_simple_exporter(exporter.clone())
            .build();
        let guard = crate::observability_guard::ObservabilityGuard {
            tracer_provider: Some(
                crate::opentelemetry_sdk_tracer_provider::OpentelemetrySdkTracerProvider::from(
                    tracer_provider,
                ),
            ),
        };
        drop(guard);
        assert!(exporter.is_shutdown_called());
    }
}
