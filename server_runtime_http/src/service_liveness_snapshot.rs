#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq, serde::Serialize,
)]
pub struct ServiceLivenessSnapshot {
    pub(super) service: crate::health_component_status::HealthComponentStatus,
}
