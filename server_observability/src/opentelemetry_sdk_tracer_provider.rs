#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    proc_macro_newtype_from_inner::FromInner,
    proc_macro_newtype_get_inner::GetInner,
)]
#[accessor(pub(super))]
#[borrow]
pub(super) struct OpentelemetrySdkTracerProvider(opentelemetry_sdk::trace::SdkTracerProvider);
impl OpentelemetrySdkTracerProvider {
    pub(super) fn shutdown(self) -> Result<(), opentelemetry_sdk::error::OTelSdkError> {
        self.0.shutdown()
    }
}
