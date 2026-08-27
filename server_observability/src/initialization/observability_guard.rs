#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug)]
pub struct ObservabilityGuard {
    pub(super) tracer_provider: Option<super::OpentelemetrySdkTracerProvider>,
}

impl ObservabilityGuard {
    pub fn shutdown(mut self) -> Result<(), super::OpentelemetrySdkObservabilityShutdownError> {
        let Some(tracer_provider) = self.tracer_provider.take() else {
            return Ok(());
        };
        tracer_provider
            .0
            .shutdown()
            .map_err(super::OpentelemetrySdkObservabilityShutdownError::from)
    }
}

impl Drop for ObservabilityGuard {
    fn drop(&mut self) {
        let Some(tracer_provider) = self.tracer_provider.take() else {
            return;
        };
        if let Err(error) = tracer_provider.0.shutdown() {
            tracing::error!(error = %error, "OpenTelemetry tracer provider shutdown failed");
        }
    }
}
