#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    generate_constructor::New,
    serde::Serialize,
)]
#[constructor(pub(crate))]
pub struct ServiceLivenessSnapshot {
    service: crate::health_component_status::HealthComponentStatus,
}
