// The owner module retains lint-sensitive semantics from the original implementation.

#[derive(optimal_memory_layout::OptimalMemoryLayout)]
#[frontend_contract::route_registry(
    state = crate::NotificationState,
    family = notification_service_contract::domain_types::NotificationRouteFamily;
    ("", "");
    schemas(
        notification_service_contract::domain_types::NotificationMessage,
        notification_service_contract::domain_types::UuidNotificationId
    );
    (
        notification_service_contract::domain_types::CreateNotificationRoute,
        super::create_notification::create_notification
    ),
)]
#[openapi()]
pub(super) struct NotificationApiRouteRegistry;
