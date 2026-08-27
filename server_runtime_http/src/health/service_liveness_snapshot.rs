#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq, serde::Serialize,
)]
pub struct ServiceLivenessSnapshot {
    pub(super) service: super::HealthComponentStatus,
}
