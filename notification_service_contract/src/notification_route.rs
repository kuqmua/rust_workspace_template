#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    proc_macro_frontend_contract_derive_route_catalog::RouteCatalog,
)]
#[route_catalog(
    family = NotificationRouteFamily,
    body_limit = crate::notification_api_body_max_bytes::NOTIFICATION_API_BODY_MAX_BYTES,
)]
pub enum NotificationRoute {
    #[route_catalog_route(crate::create_notification_route::CreateNotificationRoute)]
    Create,
}
