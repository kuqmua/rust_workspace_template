// The owner module retains lint-sensitive semantics from the original implementation.

use super::git_info_response::{__path_git_info_response, git_info_response};
use super::health::{__path_health, health};
use super::health_check::{__path_health_check, health_check};
use super::health_live::{__path_health_live, health_live};
use super::health_ready::{__path_health_ready, health_ready};

#[derive(optimal_memory_layout::OptimalMemoryLayout)]
#[frontend_contract::route_registry(
    state = crate::ArcCommonRoutesAppState,
    family = crate::CommonRouteFamily;
    ("", "");
    schemas(
        crate::HealthComponent,
        crate::HealthComponentKind,
        crate::HealthComponents,
        crate::HealthStatus
    );
    (crate::GitInfoRoute, git_info_response),
    (crate::HealthRoute, health),
    (crate::HealthCheckRoute, health_check),
    (crate::HealthLiveRoute, health_live),
    (crate::HealthReadyRoute, health_ready),
)]
#[openapi(tags((name = "service", description = "Service operational routes")))]
pub(super) struct CommonRouteRegistry;
