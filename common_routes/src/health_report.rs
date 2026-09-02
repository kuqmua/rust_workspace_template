#[derive(proc_macro_getters::Getters)]
#[getters(bare)]
#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    PartialEq,
    Eq,
    serde::Deserialize,
    serde::Serialize,
    utoipa::ToSchema,
)]
pub struct HealthReport {
    #[getters(skip)]
    components: crate::health_components::HealthComponents,
    #[getters(copy)]
    status: crate::health_status::HealthStatus,
}
impl HealthReport {
    #[cfg(test)]
    #[must_use]
    pub(crate) const fn components(&self) -> &[crate::health_component::HealthComponent] {
        self.components.as_slice()
    }

    #[must_use]
    pub fn liveness() -> Self {
        Self {
            components: crate::health_components::HealthComponents::from([
                crate::health_component::HealthComponent::new(
                    crate::health_component_kind::HealthComponentKind::ServiceAvailability,
                    crate::health_status::HealthStatus::Ok,
                ),
            ]),
            status: crate::health_status::HealthStatus::Ok,
        }
    }
    #[must_use]
    pub fn readiness(
        database_available: crate::health_database_available::HealthDatabaseAvailable,
    ) -> Self {
        let database_status = if database_available.is_available() {
            crate::health_status::HealthStatus::Ok
        } else {
            crate::health_status::HealthStatus::Error
        };
        let status = if database_available.is_available() {
            crate::health_status::HealthStatus::Ok
        } else {
            crate::health_status::HealthStatus::Degraded
        };
        Self {
            components: crate::health_components::HealthComponents::from([
                crate::health_component::HealthComponent::new(
                    crate::health_component_kind::HealthComponentKind::ServiceAvailability,
                    crate::health_status::HealthStatus::Ok,
                ),
                crate::health_component::HealthComponent::new(
                    crate::health_component_kind::HealthComponentKind::DatabaseConnectivity,
                    database_status,
                ),
            ]),
            status,
        }
    }
}
