#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    PartialEq,
    Eq,
    serde::Deserialize,
    serde::Serialize,
    utoipa::ToSchema,
)]
pub struct HealthReport {
    pub(super) components: crate::health_components::HealthComponents,
    pub(super) status: crate::health_status::HealthStatus,
}
impl HealthReport {
    #[must_use]
    pub fn liveness() -> Self {
        Self {
            components: crate::health_components::HealthComponents::from([
                crate::health_component::HealthComponent {
                    kind: crate::health_component_kind::HealthComponentKind::ServiceAvailability,
                    status: crate::health_status::HealthStatus::Ok,
                },
            ]),
            status: crate::health_status::HealthStatus::Ok,
        }
    }
    #[must_use]
    pub fn readiness(
        database_available: crate::health_database_available::HealthDatabaseAvailable,
    ) -> Self {
        let database_status = if database_available.0 {
            crate::health_status::HealthStatus::Ok
        } else {
            crate::health_status::HealthStatus::Error
        };
        let status = if database_available.0 {
            crate::health_status::HealthStatus::Ok
        } else {
            crate::health_status::HealthStatus::Degraded
        };
        Self {
            components: crate::health_components::HealthComponents::from([
                crate::health_component::HealthComponent {
                    kind: crate::health_component_kind::HealthComponentKind::ServiceAvailability,
                    status: crate::health_status::HealthStatus::Ok,
                },
                crate::health_component::HealthComponent {
                    kind: crate::health_component_kind::HealthComponentKind::DatabaseConnectivity,
                    status: database_status,
                },
            ]),
            status,
        }
    }
    #[must_use]
    pub const fn status(&self) -> crate::health_status::HealthStatus {
        self.status
    }
}
