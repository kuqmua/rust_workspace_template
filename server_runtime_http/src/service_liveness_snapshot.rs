#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    proc_macro_new::New,
    serde::Serialize,
)]
#[constructor(pub(crate))]
pub struct ServiceLivenessSnapshot {
    service: crate::health_component_status::HealthComponentStatus,
}
