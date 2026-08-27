// The owner module retains lint-sensitive semantics from the original implementation.
#![allow(clippy::arbitrary_source_item_ordering, clippy::needless_for_each)]

use super::git_info::{__path_git_info, git_info};
use super::health::{__path_health, health};
use super::health_check::{__path_health_check, health_check};
use super::health_live::{__path_health_live, health_live};
use super::health_ready::{__path_health_ready, health_ready};

#[derive(optimal_memory_layout::OptimalMemoryLayout)]
#[frontend_contract::domain_types::route_registry(
    state = crate::domain_types::ArcCommonRoutesAppState,
    family = crate::domain_types::CommonRouteFamily;
    ("", "");
    schemas(
        crate::domain_types::HealthComponent,
        crate::domain_types::HealthComponentKind,
        crate::domain_types::HealthComponents,
        crate::domain_types::HealthStatus
    );
    (crate::domain_types::GitInfoRoute, git_info),
    (crate::domain_types::HealthRoute, health),
    (crate::domain_types::HealthCheckRoute, health_check),
    (crate::domain_types::HealthLiveRoute, health_live),
    (crate::domain_types::HealthReadyRoute, health_ready),
)]
#[openapi(tags((name = "service", description = "Service operational routes")))]
pub(super) struct CommonRouteRegistry;
