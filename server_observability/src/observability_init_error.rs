#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
pub enum ObservabilityInitError {
    #[error("failed to build OTLP span exporter: {0}")]
    Exporter(crate::opentelemetry_otlp_exporter_build_error::OpentelemetryOtlpExporterBuildError),
    #[error("failed to install tracing subscriber: {0}")]
    Subscriber(crate::tracing_subscriber_init_error::TracingSubscriberInitError),
}
