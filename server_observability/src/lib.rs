mod observability;
mod observed_error;

pub use observability::{
    ObservabilityGuard, ObservabilityInitError, OpentelemetryOtlpExporterBuildError,
    OpentelemetrySdkObservabilityShutdownError, ServiceName, TracingSubscriberInitError,
    initialize_service_observability,
};
pub use observed_error::{
    ObservedError, ObservedErrorBacktrace, ObservedErrorCode, StdPanicLocation,
    TracingObservedErrorSpanTrace,
};

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq)]
pub enum ServiceTracingFormat {
    Json,
    Text,
}
