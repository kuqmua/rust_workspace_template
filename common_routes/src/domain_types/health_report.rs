#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
use super::{
    HealthComponent, HealthComponentKind, HealthComponents, HealthDatabaseAvailable, HealthStatus,
};

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
    pub(super) components: HealthComponents,
    pub(super) status: HealthStatus,
}
impl HealthReport {
    #[must_use]
    pub fn liveness() -> Self {
        Self {
            components: HealthComponents::from([HealthComponent {
                kind: HealthComponentKind::ServiceAvailability,
                status: HealthStatus::Ok,
            }]),
            status: HealthStatus::Ok,
        }
    }
    #[must_use]
    pub fn readiness(database_available: HealthDatabaseAvailable) -> Self {
        let database_status = if database_available.0 {
            HealthStatus::Ok
        } else {
            HealthStatus::Error
        };
        let status = if database_available.0 {
            HealthStatus::Ok
        } else {
            HealthStatus::Degraded
        };
        Self {
            components: HealthComponents::from([
                HealthComponent {
                    kind: HealthComponentKind::ServiceAvailability,
                    status: HealthStatus::Ok,
                },
                HealthComponent {
                    kind: HealthComponentKind::DatabaseConnectivity,
                    status: database_status,
                },
            ]),
            status,
        }
    }
    #[must_use]
    pub const fn status(&self) -> HealthStatus {
        self.status
    }
}
