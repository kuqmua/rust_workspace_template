#[derive(
    Debug,
    Clone,
    Copy,
    optimal_memory_layout::OptimalMemoryLayout,
    newtype::FromInner,
    newtype::IntoInnerFrom,
)]
pub(crate) struct HealthCheckSucceeded(bool);
