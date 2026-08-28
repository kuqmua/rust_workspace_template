use crate::domain_types::{CreateNotificationRoute, NOTIFICATION_API_BODY_MAX_BYTES};

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    frontend_contract::RouteCatalog,
)]
#[route_catalog(
    family = NotificationRouteFamily,
    body_limit = NOTIFICATION_API_BODY_MAX_BYTES,
)]
pub enum NotificationRoute {
    #[route_catalog_route(CreateNotificationRoute)]
    Create,
}
