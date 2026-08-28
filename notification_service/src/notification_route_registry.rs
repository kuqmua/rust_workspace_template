// The owner module retains lint-sensitive semantics from the original implementation.

#[derive(optimal_memory_layout::OptimalMemoryLayout)]
#[frontend_contract::endpoint_registry(
    state = crate::NotificationState;
    (
        notification_service_contract::domain_types::NotificationOperationalRoute::Metrics,
        super::metrics::metrics
    ),
    (
        notification_service_contract::domain_types::NotificationOperationalRoute::OpenApi,
        super::open_api::notification_open_api
    ),
)]
pub(super) struct NotificationRouteRegistry;
