// The owner module retains lint-sensitive semantics from the original implementation.

#[derive(optimal_memory_layout::OptimalMemoryLayout)]
#[frontend_contract_macros::route_registry(
    state = crate::notification_state::NotificationState,
    family = notification_service_contract::notification_route::NotificationRouteFamily;
    ("", "");
    schemas(
        notification_service_contract::notification_message::NotificationMessage,
        notification_service_contract::uuid_notification_id::UuidNotificationId
    );
    (
        notification_service_contract::create_notification_route::CreateNotificationRoute,
        crate::create_notification::create_notification
    ),
)]
#[openapi()]
pub(super) struct NotificationApiRouteRegistry;
