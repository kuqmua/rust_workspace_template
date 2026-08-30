#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, newtype::FromInner)]
pub(super) struct OpentelemetrySdkTracerProvider(opentelemetry_sdk::trace::SdkTracerProvider);
impl OpentelemetrySdkTracerProvider {
    pub(super) fn shutdown(self) -> Result<(), opentelemetry_sdk::error::OTelSdkError> {
        self.0.shutdown()
    }
}
