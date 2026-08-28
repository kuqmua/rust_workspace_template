pub use crate::init_service_observability::init_service_observability;
pub use crate::observability_guard::ObservabilityGuard;
pub use crate::observability_init_error::ObservabilityInitError;
pub use crate::opentelemetry_otlp_exporter_build_error::OpentelemetryOtlpExporterBuildError;
pub use crate::opentelemetry_sdk_observability_shutdown_error::OpentelemetrySdkObservabilityShutdownError;
pub(crate) use crate::opentelemetry_sdk_tracer_provider::OpentelemetrySdkTracerProvider;
pub use crate::service_name::ServiceName;
pub use crate::tracing_subscriber_init_error::TracingSubscriberInitError;

#[cfg(test)]
mod tests {
    #[test]
    fn empty_guard_shutdown_is_idempotent_and_service_name_displays() {
        let guard = super::ObservabilityGuard {
            tracer_provider: None,
        };
        guard.shutdown().expect("599ca192 empty_guard_shutdown_is_idempotent_and_service_name_displays invariant must hold");
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
        let guard = super::ObservabilityGuard {
            tracer_provider: Some(super::OpentelemetrySdkTracerProvider::from(tracer_provider)),
        };
        drop(guard);
        assert!(exporter.is_shutdown_called());
    }
}
