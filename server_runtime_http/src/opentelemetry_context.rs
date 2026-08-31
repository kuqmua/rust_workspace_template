#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    newtype::DerefInner,
    newtype::FromInner,
)]
pub struct OpentelemetryContext(opentelemetry::Context);
