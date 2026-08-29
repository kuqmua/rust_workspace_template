// The owner module retains lint-sensitive semantics from the original implementation.

#[derive(optimal_memory_layout::OptimalMemoryLayout)]
#[frontend_contract_macros::endpoint_registry(
    state = crate::notification_state::NotificationState;
    (
        notification_service_contract::notification_operational_route::NotificationOperationalRoute::Metrics,
        crate::metrics::metrics
    ),
    (
        notification_service_contract::notification_operational_route::NotificationOperationalRoute::OpenApi,
        super::notification_open_api::notification_open_api
    ),
)]
pub(super) struct NotificationRouteRegistry;
