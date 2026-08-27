#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    newtype::DerefInner,
    newtype::FromInner,
)]
pub struct OpentelemetryContext(pub(super) opentelemetry::Context);
