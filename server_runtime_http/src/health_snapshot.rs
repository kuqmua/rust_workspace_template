#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq, serde::Serialize,
)]
pub struct HealthSnapshot {
    pub(super) database: crate::health_component_status::HealthComponentStatus,
    pub(super) service: crate::health_component_status::HealthComponentStatus,
}

impl HealthSnapshot {
    #[must_use]
    pub const fn database(self) -> crate::health_component_status::HealthComponentStatus {
        self.database
    }

    #[must_use]
    pub const fn service(self) -> crate::health_component_status::HealthComponentStatus {
        self.service
    }
}
