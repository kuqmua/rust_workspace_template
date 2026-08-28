// The owner module retains lint-sensitive semantics from the original implementation.
#![allow(clippy::arbitrary_source_item_ordering, clippy::needless_for_each)]

#[derive(optimal_memory_layout::OptimalMemoryLayout)]
#[frontend_contract::domain_types::route_registry(
    state = crate::domain_types::NotificationState,
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
