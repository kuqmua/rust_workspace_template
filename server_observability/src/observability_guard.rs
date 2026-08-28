#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug)]
pub struct ObservabilityGuard {
    pub(super) tracer_provider: Option<crate::initialization::OpentelemetrySdkTracerProvider>,
}

impl ObservabilityGuard {
    pub fn shutdown(
        mut self,
    ) -> Result<(), crate::initialization::OpentelemetrySdkObservabilityShutdownError> {
        let Some(tracer_provider) = self.tracer_provider.take() else {
            return Ok(());
        };
        tracer_provider
            .0
            .shutdown()
            .map_err(crate::initialization::OpentelemetrySdkObservabilityShutdownError::from)
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
