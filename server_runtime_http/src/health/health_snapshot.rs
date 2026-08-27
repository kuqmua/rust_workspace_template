#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq, serde::Serialize,
)]
pub struct HealthSnapshot {
    pub(super) database: super::HealthComponentStatus,
    pub(super) service: super::HealthComponentStatus,
}

impl HealthSnapshot {
    #[must_use]
    pub const fn database(self) -> super::HealthComponentStatus {
        self.database
    }

    #[must_use]
    pub const fn service(self) -> super::HealthComponentStatus {
        self.service
    }
}
