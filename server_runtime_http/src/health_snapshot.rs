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
pub struct HealthSnapshot {
    database: crate::health_component_status::HealthComponentStatus,
    service: crate::health_component_status::HealthComponentStatus,
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
