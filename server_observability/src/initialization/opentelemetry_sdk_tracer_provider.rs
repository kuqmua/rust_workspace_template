#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, newtype::FromInner)]
pub(super) struct OpentelemetrySdkTracerProvider(
    pub(super) opentelemetry_sdk::trace::SdkTracerProvider,
);
