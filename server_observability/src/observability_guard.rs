#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug)]
pub struct ObservabilityGuard {
    tracer_provider:
        Option<crate::opentelemetry_sdk_tracer_provider::OpentelemetrySdkTracerProvider>,
}

impl ObservabilityGuard {
    pub fn shutdown(
        mut self,
    ) -> Result<(), crate::opentelemetry_sdk_observability_shutdown_error::OpentelemetrySdkObservabilityShutdownError>{
        let Some(tracer_provider) = self.tracer_provider.take() else {
            return Ok(());
        };
        tracer_provider
            .shutdown()
            .map_err(crate::opentelemetry_sdk_observability_shutdown_error::OpentelemetrySdkObservabilityShutdownError::from)
    }
}

impl From<Option<crate::opentelemetry_sdk_tracer_provider::OpentelemetrySdkTracerProvider>>
    for ObservabilityGuard
{
    fn from(
        tracer_provider: Option<
            crate::opentelemetry_sdk_tracer_provider::OpentelemetrySdkTracerProvider,
        >,
    ) -> Self {
        Self { tracer_provider }
    }
}

impl Drop for ObservabilityGuard {
    fn drop(&mut self) {
        let Some(tracer_provider) = self.tracer_provider.take() else {
            return;
        };
        if let Err(error) = tracer_provider.shutdown() {
            tracing::error!(error = %error, "OpenTelemetry tracer provider shutdown failed");
        }
    }
}
